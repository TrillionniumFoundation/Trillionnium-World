# trnm-world-ui-fragments

Status: current cutover candidate  
Owner: Trillionnium World Rust-owned presentation fragments

## Purpose

This crate renders bounded escaped HTML fragments for World home, keypad movement, route previews, task graphs, and action-target metadata from typed domain/projection input.

## Authority and non-goals

Fragments are presentation only. They do not authenticate users, execute commands, mutate World state, infer balances/settlement/completion, assign online sequence, or replace the native client/Nakama runtime. `data-*` attributes are untrusted client hints until a server independently validates the submitted typed request.

## Public contracts

The surface includes the fragment-renderer and Rust-owned shell contract constants, HTML/attribute escaping, bilingual visible-text helpers, keypad label/direction candidates, typed action-button views, and render functions over validated projections. Output identifies its source/contract and separates display copy from machine IDs.

## State and invariants

The crate is stateless. Every untrusted text/attribute value is escaped exactly once for its context; stable IDs and target metadata remain bounded; identical input yields identical output; disabled movement remains visibly/semantically disabled; and English/Chinese labels preserve accessibility attributes. Rendering cannot synthesize authoritative state absent from the projection.

## Dependencies and boundaries

The crate depends on domain command-decision helpers and read projections, but not server/network/database/filesystem/environment/credentials or CEX/Nakama implementations. It must not call mutation functions in response to rendering. Browser JavaScript submits typed intents to an authenticated endpoint and may not treat DOM state as authority.

## Failure and recovery

Missing/invalid typed input produces an empty, disabled, or explicitly unavailable fragment according to contract; it never emits unsafe raw markup or a false success. Rendering failure is recoverable by rebuilding from a verified projection. Client modification of HTML/attributes has no server effect without revalidation.

## Testing and evidence

Tests cover HTML/attribute escaping, quotes/control/CJK text, deterministic output, accessibility labels, disabled movement, bounded metadata, target mapping, and hostile XSS strings. Browser E2E, CSP, screen-reader, keyboard-only, localization, and human comprehension are separate evidence classes.

## Compatibility and change control

Changing IDs/classes/data attributes, target semantics, bilingual rules, accessibility behavior, or output structure requires a fragment/shell version, browser/client compatibility tests, security review, and rollback plan. A UI change cannot expand authority or public routing.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-ui-fragments-design.md`](../../../../docs/modules/world-authority/trnm-world-ui-fragments-design.md).
