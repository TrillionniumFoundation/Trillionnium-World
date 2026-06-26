#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_status.sh"

required_lines=(
  'trillionnium_world_release_review_status_v1'
  'check_trillionnium_world_release_review_quickcheck.sh'
  'release-review-status.json'
  'release-review-status.md'
  'release-review-status-quickcheck.log'
  'green: ($status != "blocked_native_bevy_replay_or_public_launch_consumption")'
  'ready_item_count: ($ready_items | length)'
  'blocked_item_count: ($blocked_items | length)'
  'public_launch_blocker_count: ($blocked_items | length)'
  'printf -- '\''- green: `%s`\n'\'' "$(jq -r '\''.green'\'' "$STATUS_JSON")"'
  'printf -- '\''- ready_item_count: `%s`\n'\'' "$(jq -r '\''.ready_item_count'\'' "$STATUS_JSON")"'
  'printf -- '\''- blocked_item_count: `%s`\n'\'' "$(jq -r '\''.blocked_item_count'\'' "$STATUS_JSON")"'
  'printf -- '\''- public_launch_blocker_count: `%s`\n\n'\'' "$(jq -r '\''.public_launch_blocker_count'\'' "$STATUS_JSON")"'
  'blocked_items'
  'ready_items'
  'Counts'
  'Still Requires Real External Evidence'
  'native_bevy_sprite_texture_sampling'
  'native_bevy_live_window_sampled_texture_correlation'
  'native_bevy_render_asset_eligibility'
  'cex_adapter_readiness'
  'CEX production world adapter readiness'
  'host_side_texture_sampling_correlation_and_render_asset_eligibility_not_gpu_upload_or_android_real_device'
  'Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.'
  'CJK/input'
  'weak-network'
  'APK resource/signature'
  'CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.'
  'android_s5_real_device_claimed: false'
  'if [[ "$REQUIRE_READY" -eq 1 && "$STATUS" != "public_launch_ready_for_review" ]]'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] release review status script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] release review status script keeps quickcheck dependency, JSON/Markdown outputs, checklist sections, require-ready guard, and Android S5 boundary"
