# Current Trillionnium World Status

Updated: 2026-08-29

Machine-readable source: `world-gap-registry-v2.json`.

## Decision

- Engineering posture: **technical alpha**.
- Player-facing posture: **pre-alpha**.
- Public online: **NO-GO**.
- Public player market: **disabled**.
- Trusted settlement promotion: **blocked pending exact-head and deployed fault evidence**.
- Canonical online authority cutover: **blocked upstream on Nakama and Integration**.

## Source state

The current stacked settlement candidate implements a transaction-free
capture/execute/apply outbox, stable remote identity, receipt lookup, live lease
fencing, exact campaign apply and operator controls. It is not yet a verified or
deployed release because exact-head Actions, independent approval, immutable
artifacts, deployed ambiguity/process-kill matrices and backup/PITR approval are
absent.

The deterministic World transition contract remains isolated on its own Draft
PR. It must prove strict canonical JSON and cross-language conformance before
Nakama shadow/cutover work can receive credit.

## Highest-priority open rows

1. Remove source-rewriting/self-modifying CI and make compiled correctness
   source directly reviewable.
2. Harden settlement worker shutdown, poison-item isolation, bounded unrelated
   concurrency and ambiguous malformed-success recovery.
3. Complete strict canonical JSON equivalence across Rust and Go.
4. Close Campaign/RTS error-path state-preservation gaps with property tests.
5. Obtain non-empty exact-head CI and independent review.
6. Enforce server-side `main` rules and required checks.
7. Complete Nakama adapter, Integration component lock and cutover rehearsal.
8. Complete deployed fault, backup/PITR, 24-hour, human and public-edge evidence.

## Evidence rule

Source implementation is not deployment evidence. Automated evidence is not
human evidence. Same-host evidence is not cross-host evidence. A partial
endurance run is not a 24-hour pass. No row is green without the exact evidence
kind specified by the release evidence contract.
