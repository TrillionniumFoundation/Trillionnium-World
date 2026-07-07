# First Contact Human Playtest Runbook - 2026-07-07

Purpose: make the local five-step First Contact playtest repeatable enough that
the first real observer run can produce useful confusion points instead of loose
notes.

## Boundary

- Status: pre-human-playtest runbook.
- This file is not beta evidence, public-launch evidence, Android S5 real-device
  evidence, production-ready UI evidence, commercial launch evidence, or human
  tester completion evidence.
- The runbook may reference local host-side Bevy runner artifacts, but it does
  not replace the observation log.

## Inputs

- Handoff packet:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json`
- Observation log:
  `docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md`
- Live screenshot:
  `acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status.png`

## Pre-Flight

1. Verify the local runner is green.
2. Refresh or inspect the classic playtest handoff packet.
3. Confirm the observation log still has three unrecorded confusion slots.
4. Keep public-launch, Android S5, beta, production-ready UI, and commercial
   launch credit false.

## Observer Rules

- One observer, one local tester, one five-step path.
- Read only the fixed prompt for each task; do not coach the tester toward the
  expected answer.
- Record the first hesitation, question, wrong target, or label-only lookup.
- Stop after the first three confusion points are recorded.
- Do not change renderer code during the run.

## Five-Step Script

| Step | Task ID | Fixed Prompt | Pass Signal | Confusion Trigger |
| --- | --- | --- | --- | --- |
| 1 | `start_campaign` | Start or continue the current local campaign. | Tester recognizes the Bevy runner campaign entry and available start/continue/replay actions. | Tester asks whether this is a web/CEX screen or cannot find the campaign entry. |
| 2 | `select_units` | Select the opening group and tell me what is selected. | Tester identifies Group 1, four units, and the worker/scout/guard/relay mix. | Tester confuses selected units with structures, shadows, or lane marks. |
| 3 | `secure_beacon` | Follow the active route and name the current target. | Tester identifies the active secure-beacon route and target as `BEACON`. | Tester follows inactive marks or cannot separate beacon objective from target/cam/queue cues. |
| 4 | `read_command_queue` | Read the next queued command. | Tester reads `move:8,4` or the equivalent queue state from the command surface. | Tester searches the center instead of using the panel, or the queue feels hidden. |
| 5 | `recover_blocked_route` | Recover from the blocked route using the visible map feedback. | Tester notices blocked-route, route-clearance, and command feedback cues. | Tester relies only on text labels or cannot see the blocked-route geometry. |

## Recording Schema

Each recorded confusion point should include:

- `task_id`: one of the five IDs above.
- `prompt`: the fixed prompt that was read.
- `observed_confusion`: what the tester did or asked.
- `visual_anchor`: the cue that should have helped.
- `likely_product_area`: value hierarchy, unit silhouette, building hierarchy,
  terrain/material grouping, objective focus, command queue, or blocked-route
  geometry.
- `next_action`: desk-review, renderer/product proposal, or no change.

## After The Run

Update the observation log by replacing the first three `unrecorded` slots with
real observer notes, then run
`scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh`.
Only those recorded notes can unlock
`ready_for_renderer_change_from_human_observation`; the runbook itself never
grants that readiness.
