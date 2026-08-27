#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-ui-human-evidence.sh"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

mkdir -p "$TEMP_ROOT/docs/evidence/ui/sessions"
cp "$ROOT_DIR/docs/evidence/ui/trnm-world-ui-human-session-v1.schema.json" \
  "$TEMP_ROOT/docs/evidence/ui/trnm-world-ui-human-session-v1.schema.json"

python3 - "$TEMP_ROOT/docs/evidence/ui/sessions/UI-HUMAN-001.json" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
session = {
    "session_id": "ui-session-negative-one",
    "participant_alias": "anon-negative-one",
    "participant_role": "independent_observer",
    "consent_obtained": True,
    "platform": "linux",
    "display": {"width": 1280, "height": 720, "scale_factor": 1.0},
    "input_mode": "hybrid",
    "accessibility": {"high_contrast": False, "subtitles": True, "low_motion": False},
    "started_at": "2026-08-28T00:00:00Z",
    "ended_at": "2026-08-28T00:01:00Z",
    "facilitator_gameplay_hints": 0,
    "five_second_answers": {
        "location_correct": True,
        "phase_correct": True,
        "objective_correct": True,
        "next_action_correct": True,
        "authority_posture_correct": True,
        "economy_posture_correct": True,
        "misread_compatibility_as_nakama": False,
        "misread_pending_as_settled": False
    },
    "vertical_slice": {
        "completed": False,
        "duration_seconds": 0,
        "stages_completed": []
    },
    "notes": "Negative fixture only."
}
packet = {
    "contract_version": "trnm_world_ui_human_session_v1",
    "claim_id": "UI-HUMAN-001",
    "evidence_status": "passed",
    "source_type": "human",
    "generated_by_automation": True,
    "exact_build": {
        "commit_sha": "1" * 40,
        "tree_sha": "2" * 40,
        "release_id": "negative-release",
        "binary_sha256": "3" * 64,
        "component_lock_id": "negative-lock"
    },
    "sessions": [session],
    "artifacts": [{
        "artifact_id": "negative-observation",
        "kind": "structured_observation",
        "sha256": "4" * 64,
        "consent_scope": "Negative fixture only."
    }],
    "limitations": ["Negative fixture only."],
    "review": {
        "reviewer": "@negative-reviewer",
        "reviewed_at": "2026-08-28T00:02:00Z",
        "decision": "approved",
        "notes": "Must be rejected."
    }
}
path.write_text(json.dumps(packet, indent=2) + "\n")
PY

if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative human evidence fixture unexpectedly accepted automation' >&2
  exit 1
fi

python3 - "$TEMP_ROOT/docs/evidence/ui/sessions/UI-HUMAN-001.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["generated_by_automation"] = False
path.write_text(json.dumps(data, indent=2) + "\n")
PY

if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative human evidence fixture unexpectedly passed with one observer' >&2
  exit 1
fi

printf '%s\n' 'TRNM UI human-evidence negative fixtures were rejected.'
