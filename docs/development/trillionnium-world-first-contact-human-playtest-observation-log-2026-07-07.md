# First Contact Human Playtest Observation Log - 2026-07-07

Purpose: prepare the local five-step First Contact playtest path so the next
renderer/product pass starts from observed confusion points instead of another
isolated micro-cue adjustment.

## Boundary

- Status: pre-human-playtest observation seed.
- This file is not beta evidence, public-launch evidence, Android S5 real-device
  evidence, production-ready UI evidence, or commercial launch evidence.
- It may use the current live host-side player-screen snapshot as desk-review
  context, but no human tester completion is claimed until an observer records
  it here.

## Source Path

- Handoff packet:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json`
- Screenshot context:
  `acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status.png`
- Current readability review:
  `docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md`

## Observation Rules

- Record the first three moments where the tester hesitates, asks what to do, or
  relies on text labels because the map state is not visually obvious.
- Do not convert a hesitation into a renderer change until it maps to one of the
  five task IDs below.
- Prefer value, silhouette, grouping, or composition fixes before touching
  already-gated exact-color micro cues.
- Preserve public-launch, Android S5, beta, and production-readiness no-claim
  boundaries.

## Five-Step Path

| Step | Task ID | Expected Signal | What To Watch |
| --- | --- | --- | --- |
| 1 | `start_campaign` | Campaign start/continue/replay actions and a campaign slot are visible. | Does the tester understand that this is already in the local Bevy runner, not a web/CEX screen? |
| 2 | `select_units` | Group 1, four selected units, and worker/scout/guard/relay roles are identifiable. | Does the selected group stand out from nearby structure shadows and lane marks? |
| 3 | `secure_beacon` | The active secure-beacon route is emphasized and the target reads as `BEACON`. | Does the active beacon objective visually dominate nearby target/cam/queue cues? |
| 4 | `read_command_queue` | The after-command queue reads as `move:8,4` and the queue panel remains readable. | Does the tester use the bottom/right panels naturally or only after searching the center? |
| 5 | `recover_blocked_route` | Blocked route, route clearance, and command feedback are visible. | Does the blocked-route recovery read from unit/path geometry, or only from text? |

## First Three Confusion Slots

1. `unrecorded`: waiting for a local observer to run the five-step path.
2. `unrecorded`: waiting for a local observer to run the five-step path.
3. `unrecorded`: waiting for a local observer to run the five-step path.

## Current Desk-Review Candidate

The current screenshot suggests a likely candidate before human observation: the
central beacon objective area has too many similarly bright accents competing
inside the same focus zone. If a tester hesitates during `secure_beacon`,
`read_command_queue`, or `recover_blocked_route`, first inspect value hierarchy,
unit/structure silhouette separation, and lane/background grouping around the
active beacon before changing any exact-color gates.

## Done When

This log has three recorded human-observed confusion points, each mapped to one
of the five task IDs, and the next proposed product/rendering change cites those
observations without changing public/S5/beta/readiness claims.
