#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
CLIENT_ROOT="$ROOT_DIR/trillionnium/crates/trnm-first-contact/src"
UI_ROOT="$CLIENT_ROOT/ui"
ACCEPTANCE="$ROOT_DIR/docs/development/trnm-world-ui-acceptance-v1.json"

fail() {
  printf 'TRNM UI contract failed: %s\n' "$*" >&2
  exit 1
}

required_files=(
  "$CLIENT_ROOT/lib.rs"
  "$UI_ROOT/mod.rs"
  "$UI_ROOT/layout.rs"
  "$UI_ROOT/model.rs"
  "$UI_ROOT/theme.rs"
  "$ROOT_DIR/docs/development/trnm-world-ui-vertical-slice-v1.md"
  "$ACCEPTANCE"
)
for file in "${required_files[@]}"; do
  [[ -f "$file" ]] || fail "required UI artifact is missing: ${file#$ROOT_DIR/}"
done

grep -q '^mod ui;$' "$CLIENT_ROOT/lib.rs" \
  || fail 'trnm-first-contact does not expose the dedicated ui module'
grep -q 'init_resource::<WorldUiState>()' "$CLIENT_ROOT/lib.rs" \
  || fail 'WorldUiState is not initialized by the native client plugin'
grep -q 'spawn_world_ui' "$CLIENT_ROOT/lib.rs" \
  || fail 'the UI control centre is not spawned'
grep -q 'sync_world_ui' "$CLIENT_ROOT/lib.rs" \
  || fail 'the UI control centre is not synchronized'

for profile in offline_world_v1 world_legacy_local_alpha_v1; do
  grep -q "$profile" "$UI_ROOT/model.rs" \
    || fail "authority profile is missing from the UI model: $profile"
done

grep -q 'UiViewportClass::Compact' "$UI_ROOT/layout.rs" \
  || fail 'compact viewport contract is missing'
grep -q 'UiViewportClass::Standard' "$UI_ROOT/layout.rs" \
  || fail 'standard viewport contract is missing'
grep -q 'UiViewportClass::Wide' "$UI_ROOT/layout.rs" \
  || fail 'wide viewport contract is missing'
grep -q 'KeyCode::F6' "$UI_ROOT/mod.rs" \
  || fail 'F6 guide visibility input is missing'
grep -q 'KeyCode::F7' "$UI_ROOT/mod.rs" \
  || fail 'F7 guide-page input is missing'
grep -q 'high_contrast_palette_preserves_maximum_text_contrast' "$UI_ROOT/theme.rs" \
  || fail 'high-contrast palette evidence is missing'
grep -q 'compatibility_snapshot_is_explicitly_noncanonical' "$UI_ROOT/model.rs" \
  || fail 'noncanonical compatibility-lab unit evidence is missing'

forbidden_pattern='MatchCompletedV1|nakama[_ -]?private[_ -]?key|match[_ -]?authority[_ -]?private[_ -]?key|nakama_canonical_claimed[[:space:]]*[:=][[:space:]]*true|public_player_market_enabled[[:space:]]*[:=][[:space:]]*true'
if grep -RniE --include='*.rs' "$forbidden_pattern" "$UI_ROOT"; then
  fail 'UI source contains a forbidden authority or public-market claim'
fi

python3 - "$ACCEPTANCE" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
if data.get("contract_version") != "trnm_world_ui_acceptance_v1":
    raise SystemExit("unexpected UI acceptance contract")
profiles = data.get("authority_profiles", {})
if profiles.get("offline") != "offline_world_v1":
    raise SystemExit("offline authority profile drift")
if profiles.get("compatibility_lab") != "world_legacy_local_alpha_v1":
    raise SystemExit("compatibility authority profile drift")
if profiles.get("nakama_canonical_claimed") is not False:
    raise SystemExit("UI acceptance may not claim Nakama canonical authority")
if data.get("public_player_market_enabled") is not False:
    raise SystemExit("public player market must remain disabled")
checks = data.get("checks")
if not isinstance(checks, list) or len(checks) < 8:
    raise SystemExit("UI acceptance matrix is incomplete")
ids = [check.get("id") for check in checks]
if len(ids) != len(set(ids)):
    raise SystemExit("duplicate UI acceptance check ID")
for check in checks:
    if check.get("state") not in {"implemented", "pending", "blocked"}:
        raise SystemExit(f"invalid UI check state: {check}")
    if check.get("evidence_kind") == "human" and check.get("state") == "implemented":
        raise SystemExit("human UI evidence cannot be closed by source automation")
PY

printf '%s\n' 'TRNM UI architecture, authority labels and acceptance contract passed.'
