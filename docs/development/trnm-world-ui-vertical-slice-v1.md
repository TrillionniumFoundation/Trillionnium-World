---
status: current
owner: trillionnium-world
applies_to:
  - native-client
  - player-facing-ui
  - deterministic-campaign
  - compatibility-lab-ui
plan_workstreams:
  - W3
  - W7
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# Trillionnium World UI Vertical Slice v1

## Decision

The next UI tranche is a player control surface layered over the existing
campaign shell and deterministic RTS HUD. It improves comprehension and
responsiveness without expanding World into a second canonical online
authority.

The UI must always describe the active runtime honestly:

- local play is `offline_world_v1`;
- the grandfathered online path is `world_legacy_local_alpha_v1` and is shown as
  a **compatibility lab**, never as Nakama-canonical public online;
- public player markets remain disabled;
- CEX-connected state may expose wallet/outbox health, but the UI cannot infer
  settlement success from a request being submitted.

## Player outcome

Within five seconds, a new observer should be able to identify:

1. where the player is;
2. which shell/game phase is active;
3. the next available action;
4. the current objective;
5. whether the session is offline or compatibility-lab attached;
6. whether economy work is local, syncing, healthy, or requires attention.

## Implemented slice

### UI module boundary

The additive `trnm-first-contact/src/ui/` module owns:

- semantic colour and interaction tokens;
- compact, standard and wide viewport classification;
- a pure, testable player-facing snapshot model;
- the persistent header, guide drawer and battle authority badge;
- F6 drawer visibility and F7 page cycling;
- campaign-overlay insets so the new chrome does not cover primary actions.

Existing campaign authority, save mutation, simulation and settlement remain in
their existing modules. The new UI does not mutate game-domain state.

### Five-second control centre

The guide drawer exposes three pages:

- **NOW** — next action, current objective and progression;
- **SYSTEM** — authority profile, exact save revision, economy/outbox health and
  status;
- **HELP** — input profile, reading order and accessibility behaviour.

The header preserves the same reading order in a compact form and automatically
reduces nonessential chips at narrower widths.

### Responsive behaviour

- **Compact** `< 960 px`: guide becomes a bottom sheet and campaign content
  reserves bottom space only while it is open.
- **Standard** `960–1439 px`: a 340 px right-side guide is reserved.
- **Wide** `>= 1440 px`: a 400 px guide and detailed authority/economy chips are
  shown.
- During RTS battle, the header/drawer are removed and replaced by a small
  noncanonical authority badge so the existing HUD remains unobstructed.

### Accessibility and input

- high-contrast mode swaps to black/white surfaces and preserves semantic
  positive/warning/critical distinctions;
- keyboard-only and mouse-only campaign behaviour remain authoritative in the
  existing flow;
- the new guide is operable by F6/F7 and real Bevy buttons;
- subtitles, controls and audio profile state are visible on the HELP page;
- UI presentation state never changes campaign authority or persisted game
  progression.

## Automated acceptance

The machine-readable matrix is
`docs/development/trnm-world-ui-acceptance-v1.json`.

The dedicated UI workflow runs:

1. static UI architecture and authority-label contract;
2. a negative fixture proving forbidden authority claims are rejected;
3. validation of every committed human-evidence packet;
4. negative fixtures proving automation and underpowered samples cannot close a
   human gate;
5. Rust formatting;
6. `trnm-first-contact` library tests;
7. Clippy with warnings denied.

## Human evidence contract

Human packets are governed by:

- `docs/evidence/ui/trnm-world-ui-human-session-v1.schema.json`;
- `docs/evidence/ui/README.md`;
- `scripts/check-trnm-ui-human-evidence.sh`;
- `scripts/test-trnm-ui-human-evidence-negative.sh`.

Packets must bind exact commit/tree/release/component-lock/binary identities,
privacy-bounded anonymous participants, consent, environment, structured
answers, artifact hashes, limitations and an independent reviewer decision.
`generated_by_automation` is fixed to `false`.

A release gate may run the validator with
`TRNM_REQUIRE_UI_HUMAN_EVIDENCE=1`; this requires all three human claims to have
approved `passed` packets. Normal source CI accepts an empty sessions directory
only as **pending**, never as passed.

## Human evidence still required

This change does not satisfy W7 human-validation rows. Promotion still requires:

- **UI-HUMAN-001:** three independent five-second-observer sessions;
- **UI-HUMAN-002:** one non-developer unguided 10–15 minute
  NEW → RPG → RTS → debrief → town run;
- **UI-HUMAN-003:** keyboard-only, mouse-only, high-contrast, subtitles on/off,
  low-motion, 1280x720 and wide-viewport sessions on supported desktop
  platforms;
- consented artifacts with exact build/commit identity and limitations.

Until those rows are attached and approved, this slice is **implemented**, not
`verified_human`, `release_ready`, or public-online approval.
