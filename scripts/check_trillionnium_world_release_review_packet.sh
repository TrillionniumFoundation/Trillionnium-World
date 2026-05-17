#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet.json"
PACKET_MD="$ACCEPTANCE_DIR/release-review-packet.md"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON" ]]; then
  PACKET_JSON="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD" ]]; then
  PACKET_MD="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD"
fi

CONVERGENCE_JSON="$ACCEPTANCE_DIR/release-review-convergence.json"
STATUS_JSON="$ACCEPTANCE_DIR/release-review-status.json"
STATUS_MD="$ACCEPTANCE_DIR/release-review-status.md"
CONVERGENCE_LOG="$ACCEPTANCE_DIR/release-review-packet-convergence.log"
INTAKE_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-intake.log"
BLOCKER_CONSISTENCY_LOG="$ACCEPTANCE_DIR/release-review-packet-blocker-consistency.log"
EVIDENCE_KIT_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-kit.log"
TEMPLATE_NEGATIVE_LOG="$ACCEPTANCE_DIR/release-review-packet-template-negative-fixtures.log"
EVIDENCE_BUNDLE_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-bundle.log"
BUNDLE_NEGATIVE_LOG="$ACCEPTANCE_DIR/release-review-packet-bundle-negative-fixtures.log"
MAP_MODELING_GATE_LOG="$ACCEPTANCE_DIR/release-review-packet-map-modeling-gate.log"
CEX_ADAPTER_LOG="$ACCEPTANCE_DIR/release-review-packet-cex-adapter-readiness.log"
CHECKPOINT_MANIFEST_LOG="$ACCEPTANCE_DIR/release-review-packet-checkpoint-manifest.log"
ARTIFACTS_FILE="$(mktemp)"
trap 'rm -f "$ARTIFACTS_FILE"' EXIT

mkdir -p "$ACCEPTANCE_DIR"

"$ROOT/scripts/check_trillionnium_world_release_review_convergence.sh" >"$CONVERGENCE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >"$INTAKE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh" >"$BLOCKER_CONSISTENCY_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$EVIDENCE_KIT_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh" >"$TEMPLATE_NEGATIVE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh" >"$EVIDENCE_BUNDLE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh" >"$BUNDLE_NEGATIVE_LOG"
"$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh" >"$MAP_MODELING_GATE_LOG"
"$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh" >"$CEX_ADAPTER_LOG"
"$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh" >"$CHECKPOINT_MANIFEST_LOG"

artifact() {
  local id="$1"
  local label="$2"
  local path="$3"
  local role="$4"
  local file_status="missing"
  local sha256=""
  local bytes=""
  local contract_version=""
  local status=""

  if [[ -f "$path" ]]; then
    file_status="present"
    sha256="$(sha256sum "$path" | awk '{print $1}')"
    bytes="$(wc -c <"$path" | tr -d ' ')"
    if [[ "$path" == *.json ]]; then
      contract_version="$(jq -r '.contract_version // empty' "$path" 2>/dev/null || true)"
      status="$(jq -r '.status // .overall_status // empty' "$path" 2>/dev/null || true)"
    fi
  fi

  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg path "$path" \
    --arg role "$role" \
    --arg file_status "$file_status" \
    --arg sha256 "$sha256" \
    --arg bytes "$bytes" \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    '{
      id: $id,
      label: $label,
      path: $path,
      role: $role,
      file_status: $file_status,
      sha256: (if $sha256 == "" then null else $sha256 end),
      bytes: (if $bytes == "" then null else ($bytes | tonumber) end),
      contract_version: (if $contract_version == "" then null else $contract_version end),
      status: (if $status == "" then null else $status end)
    }' >>"$ARTIFACTS_FILE"
}

artifact native_bevy_keyboard_replay "Native/Bevy keyboard replay" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json" release_review_input
artifact native_bevy_action_coach "Native/Bevy action coach" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json" release_review_input
artifact native_bevy_player_hud_debug_layer "Native/Bevy player HUD/debug layer" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json" release_review_input
artifact native_bevy_player_ui_rescue "Native/Bevy player UI rescue" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-ui-rescue.json" release_review_input
artifact native_bevy_live_window_screenshot_sequence "Native/Bevy live-window screenshot sequence" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json" release_review_input
artifact native_bevy_sprite_texture_sampling "Native/Bevy sprite texture sampling" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json" release_review_input
artifact native_bevy_live_window_sampled_texture_correlation "Native/Bevy live-window sampled texture correlation" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json" release_review_input
artifact native_bevy_render_asset_eligibility "Native/Bevy render asset eligibility" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json" release_review_input
artifact cex_adapter_readiness "CEX production world adapter readiness" "$ROOT/acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness.json" release_review_input
artifact s5_real_device_evidence "S5 real-device evidence validation" "$ROOT/acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json" release_review_input
artifact public_launch_readiness "Public launch readiness" "$ACCEPTANCE_DIR/public-launch-readiness.json" release_review_input
artifact public_launch_evidence_intake "Public launch evidence intake" "$ACCEPTANCE_DIR/public-launch-evidence-intake.json" release_review_input
artifact production_map_pack_public_evidence_collection "Production map-pack public evidence collection" "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence-collection.json" release_review_collection
artifact map_modeling_gate "Map modeling gate" "$ROOT/acceptance/S4_map_pack_gate/latest/map-modeling-gate.json" release_review_input
artifact cohort_commercial_evidence_collection "Cohort/commercial evidence collection" "$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.json" release_review_collection
artifact external_ops_evidence_collection "External ops evidence collection" "$ACCEPTANCE_DIR/external-ops-evidence-collection.json" release_review_collection
artifact public_launch_blocker_consistency "Public launch blocker consistency" "$ACCEPTANCE_DIR/public-launch-blocker-consistency.json" release_review_gate
artifact public_launch_evidence_kit "Public launch evidence kit" "$ACCEPTANCE_DIR/public-launch-evidence-kit.json" release_review_gate
artifact public_launch_template_negative_fixtures "Public launch template negative fixtures" "$ACCEPTANCE_DIR/public-launch-template-negative-fixtures.json" release_review_gate
artifact public_launch_evidence_bundle "Public launch evidence bundle" "$ACCEPTANCE_DIR/public-launch-evidence-bundle.json" release_review_gate
artifact public_launch_bundle_negative_fixtures "Public launch bundle negative fixtures" "$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures.json" release_review_gate
artifact public_launch_status_only_fixture_guard "Public launch status-only fixture guard" "$ACCEPTANCE_DIR/public-launch-status-only-fixtures.json" release_review_gate
artifact production_map_pack_public_evidence "Production map-pack public evidence" "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json" release_review_input
artifact cohort_commercial_evidence "Cohort/commercial evidence validation" "$ACCEPTANCE_DIR/cohort-commercial-evidence.json" release_review_input
artifact external_ops_evidence "External ops evidence validation" "$ACCEPTANCE_DIR/external-ops-evidence.json" release_review_input
artifact release_signoff_summary "Release signoff summary" "$ACCEPTANCE_DIR/release-signoff-summary.json" release_review_input
artifact release_review_quickcheck "Release review quickcheck" "$ACCEPTANCE_DIR/release-review-quickcheck.json" release_review_input
artifact release_review_status_json "Release review status JSON" "$STATUS_JSON" release_review_checklist
artifact release_review_status_markdown "Release review status Markdown" "$STATUS_MD" release_review_checklist
artifact release_review_convergence "Release review convergence" "$CONVERGENCE_JSON" release_review_gate
artifact release_review_checkpoint_manifest "Release review checkpoint manifest" "$ACCEPTANCE_DIR/release-review-checkpoint-manifest.json" release_review_checkpoint
artifact release_review_packet_convergence_log "Release review packet convergence log" "$CONVERGENCE_LOG" release_review_log

ARTIFACTS_JSON="$(jq -s '.' "$ARTIFACTS_FILE")"
CONVERGENCE_GREEN="$(jq -r '.green // false' "$CONVERGENCE_JSON")"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$STATUS_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$STATUS_JSON")"
STATUS_READY="$(jq -r '.status // "unknown"' "$STATUS_JSON")"
CONVERGENCE_STATUS="$(jq -r '.status // "unknown"' "$CONVERGENCE_JSON")"
BLOCKED_ITEMS_JSON="$(jq -c '.blocked_items // []' "$STATUS_JSON")"
READY_ITEMS_JSON="$(jq -c '.ready_items // []' "$STATUS_JSON")"
MISSING_ARTIFACTS_JSON="$(jq -c '[.[] | select(.file_status != "present") | .id]' <<<"$ARTIFACTS_JSON")"
MISSING_ARTIFACT_COUNT="$(jq 'length' <<<"$MISSING_ARTIFACTS_JSON")"

PACKET_STATUS=release_review_packet_blocked
if [[ "$CONVERGENCE_GREEN" == "true" && "$READY_FOR_RELEASE_REVIEW" == "true" && "$MISSING_ARTIFACT_COUNT" == "0" ]]; then
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    PACKET_STATUS=release_review_packet_green
  else
    PACKET_STATUS=release_review_packet_ready_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_packet_v1" \
  --arg status "$PACKET_STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg markdown_path "$PACKET_MD" \
  --arg convergence_status "$CONVERGENCE_STATUS" \
  --arg status_checklist_status "$STATUS_READY" \
  --argjson artifacts "$ARTIFACTS_JSON" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson blocked_items "$BLOCKED_ITEMS_JSON" \
  --argjson ready_items "$READY_ITEMS_JSON" \
  --argjson missing_artifacts "$MISSING_ARTIFACTS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_packet",
    markdown_path: $markdown_path,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    convergence_status: $convergence_status,
    status_checklist_status: $status_checklist_status,
    packet_rule: "refresh_release_review_convergence_then_emit_a_checksummed_review_manifest_for_operator_and_reviewer_handoff",
    artifacts: $artifacts,
    missing_artifacts: $missing_artifacts,
    ready_items: $ready_items,
    blocked_items: $blocked_items,
    reviewer_next_action: (if $public_launch_ready then "review_public_launch_ready_evidence" else "collect_real_external_public_launch_evidence" end)
  }' >"$PACKET_JSON"

{
  printf '# Trillionnium World Release Review Packet\n\n'
  printf -- '- status: `%s`\n' "$PACKET_STATUS"
  printf -- '- ready_for_release_review: `%s`\n' "$READY_FOR_RELEASE_REVIEW"
  printf -- '- public_launch_ready: `%s`\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- android_s5_real_device_claimed: `false`\n'
  printf -- '- proof_scope: `host_side_bevy_runtime_replay_not_android_real_device`\n\n'
  printf '## Evidence Artifacts\n\n'
  jq -r '.artifacts[] | "- `\(.id)`: \(.path)\n  - role: `\(.role)`\n  - file_status: `\(.file_status)`\n  - contract_version: `\(.contract_version // "n/a")`\n  - status: `\(.status // "n/a")`\n  - sha256: `\(.sha256 // "missing")`\n  - bytes: `\(.bytes // 0)`"' "$PACKET_JSON"
  printf '\n## Green For Review\n\n'
  jq -r '.ready_items[] | "- [\(.ready | if . then "x" else " " end)] \(.label): \(.detail)"' "$PACKET_JSON"
  printf '\n## Still Requires Real External Evidence\n\n'
  jq -r 'if (.blocked_items | length) == 0 then "- [x] No public-launch blockers remain." else .blocked_items[] | "- [ ] \(.label): \(.needed)" end' "$PACKET_JSON"
  printf '\n## Boundary\n\n'
  printf -- '- Native/Bevy replay, action coach, HUD/debug layer, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
  printf -- '- CEX adapter readiness proves the current CEX incubator exports the Trillionnium world runtime adapter contract; it is not a substitute for real external public-launch evidence.\n'
  printf -- '- The checkpoint manifest groups the current dirty working tree for review; it does not stage, commit, or publish anything.\n'
  printf -- '- Public launch remains blocked until the external evidence above is attached.\n'
} >"$PACKET_MD"

case "$PACKET_STATUS" in
  release_review_packet_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_GREEN %s %s\n' "$PACKET_JSON" "$PACKET_MD"
    ;;
  release_review_packet_ready_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_READY_WITH_PUBLIC_LAUNCH_BLOCKERS %s %s\n' "$PACKET_JSON" "$PACKET_MD"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_BLOCKED %s %s\n' "$PACKET_STATUS" "$PACKET_JSON" >&2
    exit 1
    ;;
esac
