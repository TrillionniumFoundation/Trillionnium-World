---
status: current-candidate
owner: trillionnium-world-ui-fragments
module: trnm-world-ui-fragments
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-ui-fragments detailed design

## Scope and authority

This crate turns validated World-domain and projection values into bounded Rust-owned HTML fragments for the standalone cutover review surface: home summary, keypad movement, route previews, task graphs and action-target metadata. It owns escaping and stable presentation contracts only. It cannot authenticate a user, execute a command, mutate World state, infer settlement/completion, assign canonical online order, or replace the native client and Nakama runtime.

## Public interfaces

The surface includes the renderer and shell contract constants; context-specific HTML and attribute escaping; bilingual visible-text and route-label helpers; keypad direction/label candidates; typed action-button view data; and render functions for home, routes, task graphs and movement controls. Output carries explicit `data-*` source/contract identifiers and machine IDs separately from display copy. Client-generated or modified DOM is never accepted by the server without typed validation.

## State model and invariants

The crate is stateless and deterministic. Identical typed input yields identical output. Every untrusted text and attribute is escaped for exactly its destination context; raw markup from a projection is not executed. IDs, labels, metadata and output bytes are bounded. Disabled/blocked movement remains both visually and semantically disabled. Bilingual labels and accessibility names are preserved. Rendering cannot add a task, route, target, result, balance or authority claim missing from its input.

## Dependency direction

The package may consume `trnm-world-domain`, deterministic command-decision helpers and `trnm-world-projection` types. It must not import server sockets, HTTP request parsing, repositories/databases, filesystem/environment/time, credentials, CEX or Nakama implementations. It may ask pure helpers whether an authored transition is available for display, but cannot perform or persist that transition. Browser code submits a typed intent to the API/server layer.

## Execution, concurrency and cancellation

Rendering is synchronous, side-effect free and safe for independent concurrent calls. It owns no locks or tasks. A server derives one verified projection, renders into a private bounded string, and publishes only the complete response. Cancellation or allocation/formatting failure drops the private result. Rendering never holds a mutable World transaction or remote call. Streaming partial HTML is not authoritative and cannot advance any revision or command cursor.

## Persistence and durability

The crate performs no persistence. Rendered fragments are disposable caches keyed by exact projection/source and fragment versions. Cache loss has no authority effect. A persisted/static response must retain source revision/digest and be invalidated on any projection or fragment change. User-edited HTML, browser storage and screenshots are evidence of presentation only, not durable state, identity, online completion or settlement.

## Idempotency, retry and failure taxonomy

Rendering the same input/version is naturally idempotent. Failures include missing/unsupported projection contract, invalid or oversized identifiers/text, unavailable target, invalid bilingual copy, escaping/formatting error and exceeded output budget. An unavailable action is rendered disabled or omitted according to the contract, never as an accepted command. Retrying after rebuilding the same verified projection is safe. Client mutation of attributes cannot bypass server-side identity/revision/command validation.

## Versioning and migration

Fragment renderer, Rust-owned shell, projection schema, target/action semantics, CSS/DOM IDs/classes, bilingual/localization policy and browser/client compatibility are distinct versions. Changing structural markup, `data-*` attributes, action target meaning, escaping, localization or accessibility behavior requires a versioned contract, browser/E2E/security/accessibility tests, cache invalidation and rollback. A presentation revision may not create a new authority or public route.

## Resource and performance budgets

Input collection counts, text/ID lengths, nested task graph/route items and final fragment bytes are bounded before formatting. Rendering is linear in bounded input and avoids repeated concatenation patterns with unbounded quadratic growth. Output and per-request allocation/latency are measured at maximum valid inputs. Server response/body, compression and CSP budgets are stricter or equal. Large lists require pagination/virtualization at the projection/API layer rather than unlimited HTML.

## Observability and security

Telemetry records fragment/projection/source versions, output type/item count/bytes/duration and bounded failure code. It excludes credentials, tokens and unnecessary private text. Tests and deployment enforce HTML/attribute escaping, content-security policy, safe link/action handling, no inline authority interpretation and no reflected unbounded diagnostics. XSS, DOM clobbering, spoofed disabled state and localization ambiguity are security defects; server revalidation remains the final control.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-ui-fragments/src/lib.rs`. Normative neighbors include ADR-0003, the projection/API/server designs, authority contract JSON and the browser-parity tests exercised by `scripts/check-trnm-world-authority-cutover.sh`. Tests cover quotes/control/CJK/HTML/script payloads, attribute/text escaping, deterministic output, disabled movement, target mapping, output bounds and bilingual/accessibility attributes. Browser, CSP, screen-reader and human usability evidence remains separate.

## Change checklist and open work

A change identifies affected fragment/projection/action/accessibility versions; updates DOM/attribute dictionaries, examples, cache rules and browser handlers; adds XSS/DOM/localization/keyboard/screen-reader and maximum-size tests; verifies no mutation/authority path; and obtains UI/security review. Open work includes generated fragment schema/snapshots, CSP deployment conformance, complete localization key management, screen-reader/keyboard-only sessions, browser compatibility and retirement when the canonical player surface is owned by the native client/Nakama path.
