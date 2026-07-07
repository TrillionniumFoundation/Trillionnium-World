# First Contact Readability Review - 2026-07-07

Purpose: convert the current live First Contact player-screen evidence into a
whole-screen product review target before any more renderer micro-cue work.

## Source Snapshot

- Source artifact: `acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status.png`
- Snapshot dimensions: `1280x699`
- Runtime source: `trillionnium-bevy-playtest.service`
- Current room/title: `first-contact-basin`
- Boundary: this is local host-side review evidence only. It does not grant
  public-launch, Android S5 real-device, beta, or production-ready UI credit.

## What Reads Well

- The side HUD is scan-friendly: resource counters, production slots, build
  palette, tactics, and order queue are compact without collapsing into noise.
- Lane anchors are legible as map structure, especially the vertical center lane
  and the horizontal lower lane.
- The selected group and order state are understandable through the bottom panel
  labels and the in-field text stack.
- The active path now avoids showing every possible opening route at once.

## Main Product Risk

The central beacon fight is still the dominant whole-screen readability risk.
The issue is not one color family or one oversized cue anymore; it is the number
of similarly bright micro accents competing inside the same central objective
area. At a glance, the active beacon, selected group, target callout, route
warning, nearby structure silhouettes, and command feedback all ask for focus at
the same time.

## Review Findings

- Unit silhouettes: units are readable in isolation, but central units lose
  hierarchy when they overlap structure shadows, beacon accents, and lane marks.
- Building hierarchy: edge structures read better than central structures; the
  central beacon and nearby command/production forms compete because their
  accents share similar brightness.
- Terrain/material grouping: lanes are strong, but center-lane floor fragments,
  road scars, and spark accents should behave as grouped background material,
  not as independent focal marks.
- Objective focus: `SECURE RELAY BEACON` is textually clear, but the active
  objective field does not visually dominate the nearby target/cam/queue cues.
- Combat flow: attack and target feedback are present, but the blocked-route
  situation is easier to read from the text stack than from unit/path geometry.

## Candidate Next Slice

Do a product-level silhouette and composition pass around the active center
objective:

- Preserve the existing exact-color gates and no-claim boundaries.
- Reduce background-value competition around the active beacon radius.
- Give selected units and the active objective a consistent value/z priority
  over structure shadows and terrain fragments.
- Keep lane anchors visible as structure, but avoid letting center-lane material
  read as additional command feedback.
- Use the five-step human playtest path to log the first three confusion points
  before choosing a renderer change.
- Seed the observation pass with
  `docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md`.

## Non-Goals

- Do not keep shaving already-gated micro cues without a fresh screenshot-visible
  reason.
- Do not add broad overlays or broad `classic_draw_scene` changes.
- Do not change gameplay, network, data, runtime-core simulation, public launch,
  Android S5, beta, or production readiness claims.

## Done When

A live player-screen review can identify, within a few seconds, the selected
group, active beacon objective, blocked route, and next command without relying
primarily on the text stack.
