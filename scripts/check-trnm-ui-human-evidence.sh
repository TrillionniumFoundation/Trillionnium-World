#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
SCHEMA="$ROOT_DIR/docs/evidence/ui/trnm-world-ui-human-session-v1.schema.json"
SESSIONS_DIR="$ROOT_DIR/docs/evidence/ui/sessions"
REQUIRE_ALL="${TRNM_REQUIRE_UI_HUMAN_EVIDENCE:-0}"

fail() {
  printf 'TRNM UI human evidence failed: %s\n' "$*" >&2
  exit 1
}

[[ -f "$SCHEMA" ]] || fail "missing human evidence schema: ${SCHEMA#$ROOT_DIR/}"
[[ "$REQUIRE_ALL" == "0" || "$REQUIRE_ALL" == "1" ]] \
  || fail 'TRNM_REQUIRE_UI_HUMAN_EVIDENCE must be 0 or 1'

python3 - "$SCHEMA" "$SESSIONS_DIR" "$REQUIRE_ALL" <<'PY'
from __future__ import annotations

import datetime as dt
import json
import pathlib
import re
import sys

schema_path = pathlib.Path(sys.argv[1])
sessions_dir = pathlib.Path(sys.argv[2])
require_all = sys.argv[3] == "1"

sha40 = re.compile(r"^[0-9a-f]{40}$")
sha64 = re.compile(r"^[0-9a-f]{64}$")
session_id_pattern = re.compile(r"^ui-session-[0-9a-z-]+$")
participant_pattern = re.compile(r"^anon-[0-9a-z-]+$")
reviewer_pattern = re.compile(r"^@[A-Za-z0-9-]+$")
all_stages = {
    "new_campaign",
    "character_confirmation",
    "rpg_mentor_loop",
    "mission_preparation",
    "rts_battle",
    "debrief",
    "town_return",
}


def reject(message: str) -> None:
    raise SystemExit(message)


def expect(condition: bool, message: str) -> None:
    if not condition:
        reject(message)


def parse_time(value: object, field: str) -> dt.datetime:
    expect(isinstance(value, str) and value, f"{field} must be a timestamp")
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as error:
        reject(f"{field} is not ISO-8601: {error}")
    expect(parsed.tzinfo is not None, f"{field} must include a timezone")
    return parsed


schema = json.loads(schema_path.read_text())
expect(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "unexpected schema draft")
properties = schema.get("properties", {})
expect(properties.get("contract_version", {}).get("const") == "trnm_world_ui_human_session_v1", "schema contract drift")
expect(properties.get("source_type", {}).get("const") == "human", "human source constraint drift")
expect(properties.get("generated_by_automation", {}).get("const") is False, "automation exclusion drift")

packet_paths = sorted(sessions_dir.glob("*.json")) if sessions_dir.exists() else []
if not packet_paths:
    if require_all:
        reject("no reviewed human UI evidence packets are present")
    print("TRNM UI human evidence: no packets present; human gates remain pending.")
    raise SystemExit(0)

passed_claims: set[str] = set()
seen_session_ids: set[str] = set()

for packet_path in packet_paths:
    packet = json.loads(packet_path.read_text())
    prefix = packet_path.as_posix()
    expect(packet.get("contract_version") == "trnm_world_ui_human_session_v1", f"{prefix}: contract version mismatch")
    claim_id = packet.get("claim_id")
    expect(claim_id in {"UI-HUMAN-001", "UI-HUMAN-002", "UI-HUMAN-003"}, f"{prefix}: invalid claim_id")
    status = packet.get("evidence_status")
    expect(status in {"recorded", "reviewed", "passed", "failed"}, f"{prefix}: invalid evidence_status")
    expect(packet.get("source_type") == "human", f"{prefix}: source_type must be human")
    expect(packet.get("generated_by_automation") is False, f"{prefix}: automated evidence is prohibited")

    build = packet.get("exact_build")
    expect(isinstance(build, dict), f"{prefix}: exact_build is required")
    expect(sha40.fullmatch(str(build.get("commit_sha", ""))) is not None, f"{prefix}: invalid commit_sha")
    expect(sha40.fullmatch(str(build.get("tree_sha", ""))) is not None, f"{prefix}: invalid tree_sha")
    expect(sha64.fullmatch(str(build.get("binary_sha256", ""))) is not None, f"{prefix}: invalid binary_sha256")
    expect(len(str(build.get("release_id", ""))) >= 8, f"{prefix}: release_id is not bound")
    expect(len(str(build.get("component_lock_id", ""))) >= 8, f"{prefix}: component_lock_id is not bound")

    sessions = packet.get("sessions")
    expect(isinstance(sessions, list) and sessions, f"{prefix}: sessions must be non-empty")
    participant_aliases: set[str] = set()
    for index, session in enumerate(sessions):
        context = f"{prefix}: sessions[{index}]"
        expect(isinstance(session, dict), f"{context}: session must be an object")
        session_id = session.get("session_id")
        alias = session.get("participant_alias")
        expect(isinstance(session_id, str) and session_id_pattern.fullmatch(session_id), f"{context}: invalid session_id")
        expect(session_id not in seen_session_ids, f"{context}: duplicate session_id across packets")
        seen_session_ids.add(session_id)
        expect(isinstance(alias, str) and participant_pattern.fullmatch(alias), f"{context}: invalid anonymous participant alias")
        expect(alias not in participant_aliases, f"{context}: duplicate participant in one packet")
        participant_aliases.add(alias)
        expect(session.get("participant_role") in {"independent_observer", "non_developer"}, f"{context}: invalid participant role")
        expect(session.get("consent_obtained") is True, f"{context}: consent is required")
        expect(session.get("platform") in {"linux", "windows", "macos"}, f"{context}: unsupported platform")
        display = session.get("display")
        expect(isinstance(display, dict), f"{context}: display is required")
        expect(isinstance(display.get("width"), int) and 640 <= display["width"] <= 7680, f"{context}: invalid display width")
        expect(isinstance(display.get("height"), int) and 480 <= display["height"] <= 4320, f"{context}: invalid display height")
        expect(isinstance(display.get("scale_factor"), (int, float)) and 0 < display["scale_factor"] <= 4, f"{context}: invalid scale factor")
        expect(session.get("input_mode") in {"hybrid", "keyboard_only", "mouse_only"}, f"{context}: invalid input mode")
        accessibility = session.get("accessibility")
        expect(isinstance(accessibility, dict), f"{context}: accessibility profile is required")
        for key in ("high_contrast", "subtitles", "low_motion"):
            expect(isinstance(accessibility.get(key), bool), f"{context}: accessibility.{key} must be boolean")
        started = parse_time(session.get("started_at"), f"{context}.started_at")
        ended = parse_time(session.get("ended_at"), f"{context}.ended_at")
        expect(ended > started, f"{context}: ended_at must be after started_at")
        hints = session.get("facilitator_gameplay_hints")
        expect(isinstance(hints, int) and hints >= 0, f"{context}: invalid facilitator hint count")
        answers = session.get("five_second_answers")
        expect(isinstance(answers, dict), f"{context}: five_second_answers are required")
        for key in (
            "location_correct",
            "phase_correct",
            "objective_correct",
            "next_action_correct",
            "authority_posture_correct",
            "economy_posture_correct",
            "misread_compatibility_as_nakama",
            "misread_pending_as_settled",
        ):
            expect(isinstance(answers.get(key), bool), f"{context}: answer {key} must be boolean")
        vertical = session.get("vertical_slice")
        expect(isinstance(vertical, dict), f"{context}: vertical_slice is required")
        expect(isinstance(vertical.get("completed"), bool), f"{context}: vertical completion must be boolean")
        duration = vertical.get("duration_seconds")
        expect(isinstance(duration, int) and 0 <= duration <= 3600, f"{context}: invalid duration")
        stages = vertical.get("stages_completed")
        expect(isinstance(stages, list) and set(stages).issubset(all_stages), f"{context}: invalid stage list")
        expect(isinstance(session.get("notes"), str) and session["notes"].strip(), f"{context}: notes are required")

    artifacts = packet.get("artifacts")
    expect(isinstance(artifacts, list) and artifacts, f"{prefix}: artifacts must be non-empty")
    for index, artifact in enumerate(artifacts):
        context = f"{prefix}: artifacts[{index}]"
        expect(isinstance(artifact, dict), f"{context}: artifact must be an object")
        expect(isinstance(artifact.get("artifact_id"), str) and artifact["artifact_id"].strip(), f"{context}: artifact_id is required")
        expect(artifact.get("kind") in {
            "structured_observation",
            "interaction_timeline",
            "screenshot",
            "video",
            "metrics_export",
            "review_record",
        }, f"{context}: invalid artifact kind")
        expect(sha64.fullmatch(str(artifact.get("sha256", ""))) is not None, f"{context}: invalid artifact sha256")
        expect(isinstance(artifact.get("consent_scope"), str) and artifact["consent_scope"].strip(), f"{context}: consent scope is required")

    limitations = packet.get("limitations")
    expect(isinstance(limitations, list) and limitations, f"{prefix}: limitations must be non-empty")
    expect(all(isinstance(item, str) and item.strip() for item in limitations), f"{prefix}: empty limitation")
    review = packet.get("review")
    expect(isinstance(review, dict), f"{prefix}: review is required")
    expect(reviewer_pattern.fullmatch(str(review.get("reviewer", ""))) is not None, f"{prefix}: invalid reviewer")
    parse_time(review.get("reviewed_at"), f"{prefix}: review.reviewed_at")
    expect(review.get("decision") in {"pending", "approved", "rejected"}, f"{prefix}: invalid review decision")
    expect(isinstance(review.get("notes"), str) and review["notes"].strip(), f"{prefix}: review notes are required")
    if status == "passed":
        expect(review.get("decision") == "approved", f"{prefix}: passed evidence requires approval")

        if claim_id == "UI-HUMAN-001":
            expect(len(sessions) >= 3, f"{prefix}: UI-HUMAN-001 requires three observers")
            expect(all(session["participant_role"] == "independent_observer" for session in sessions), f"{prefix}: UI-HUMAN-001 requires independent observers")
            expect(all(session["five_second_answers"][key] for session in sessions for key in ("phase_correct", "objective_correct", "next_action_correct")), f"{prefix}: every observer must identify phase, objective and next action")
            expect(sum(session["five_second_answers"]["authority_posture_correct"] for session in sessions) >= 2, f"{prefix}: insufficient authority comprehension")
            expect(sum(session["five_second_answers"]["economy_posture_correct"] for session in sessions) >= 2, f"{prefix}: insufficient economy comprehension")
            expect(not any(session["five_second_answers"]["misread_compatibility_as_nakama"] for session in sessions), f"{prefix}: compatibility was misread as Nakama canonical")
            expect(not any(session["five_second_answers"]["misread_pending_as_settled"] for session in sessions), f"{prefix}: pending CEX work was misread as settled")

        elif claim_id == "UI-HUMAN-002":
            eligible = [session for session in sessions if session["participant_role"] == "non_developer"]
            expect(eligible, f"{prefix}: UI-HUMAN-002 requires a non-developer")
            expect(any(
                session["facilitator_gameplay_hints"] == 0
                and session["vertical_slice"]["completed"] is True
                and 600 <= session["vertical_slice"]["duration_seconds"] <= 900
                and set(session["vertical_slice"]["stages_completed"]) == all_stages
                for session in eligible
            ), f"{prefix}: no unguided 10-15 minute complete vertical slice")

        elif claim_id == "UI-HUMAN-003":
            modes = {session["input_mode"] for session in sessions}
            expect({"keyboard_only", "mouse_only"}.issubset(modes), f"{prefix}: keyboard-only and mouse-only coverage required")
            expect(any(session["accessibility"]["high_contrast"] for session in sessions), f"{prefix}: high-contrast coverage required")
            expect({session["accessibility"]["subtitles"] for session in sessions} == {False, True}, f"{prefix}: subtitles on/off coverage required")
            expect(any(session["accessibility"]["low_motion"] for session in sessions), f"{prefix}: low-motion coverage required")
            expect(any(session["display"]["width"] == 1280 and session["display"]["height"] == 720 for session in sessions), f"{prefix}: 1280x720 coverage required")
            expect(any(session["display"]["width"] >= 1440 for session in sessions), f"{prefix}: wide viewport coverage required")

        passed_claims.add(claim_id)

if require_all:
    required = {"UI-HUMAN-001", "UI-HUMAN-002", "UI-HUMAN-003"}
    expect(passed_claims == required, f"passed human claims are incomplete: {sorted(passed_claims)}")

print(f"TRNM UI human evidence: validated {len(packet_paths)} packet(s); passed={sorted(passed_claims)}")
PY
