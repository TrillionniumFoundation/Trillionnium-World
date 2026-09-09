# trnm-world-command

Status: current cutover candidate  
Owner: Trillionnium World deterministic command decisions

## Purpose

This crate defines typed World-domain commands, stable decisions and rejection reasons, movement/transition resolution, and tactics/task/item/skill state transitions over a `WorldState` candidate.

## Authority and non-goals

It decides deterministic World business semantics only. It does not authenticate a caller, assign the canonical online order, persist idempotency, manage reconnect generations, hold database locks, perform remote I/O, sign completion, mutate CEX wallet state, or replace Nakama as the canonical online authority. An admitted Nakama or bounded migration adapter supplies ordering and identity separately.

## Public contracts

The principal surface includes `WorldCommand`, `WorldCommandDecision`, typed tactics request/outcome values, `apply_command`, `apply_tactics_command`, and transition-decision helpers. Each result carries stable status/reason/contract material sufficient for a caller to persist an accepted successor or expose a bounded rejection without interpreting display text.

## State and invariants

A command executes against one expected aggregate revision. It either produces a completely valid successor with a deterministic decision or leaves the input state semantically unchanged. Movement follows authored graph/transition rules; tasks, items, skills, health/resources, and character state remain bounded and referentially valid. A command cannot smuggle participant, custody, or canonical ordering authority into World fields.

## Dependencies and boundaries

The crate depends on `trnm-world-domain` and deterministic support libraries only. It must not import projections, UI, API/server, SQL, filesystem, HTTP, environment, credentials, CEX, or Nakama implementations. Application adapters validate caller identity/order/revision before invoking the command and persist only after full success.

## Failure and recovery

Unknown commands, invalid subjects/targets, blocked transitions, unmet prerequisites, stale expected state, insufficient resources, invalid tactics, and crossed versions fail closed with stable bounded reasons. Retrying identical admitted bytes against the identical revision is deterministic. Altered duplicates or stale revisions are rejected by the owning application/repository boundary.

## Testing and evidence

Tests cover every command family, success and rejection state hashes, graph movement, task lifecycle, tactics/skill/item bounds, duplicate/stale scenarios at the application boundary, deterministic repetition, and hostile inputs. Nakama shadow comparison and PostgreSQL replay remain higher evidence classes.

## Compatibility and change control

Changing command fields, error precedence, transition rules, arithmetic, side effects, or canonical outcome material requires a version bump, vectors, migration/rollback analysis, projection/API updates, and cross-repository conformance. Adding a command does not authorize a new public route by itself.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-command-design.md`](../../../../docs/modules/world-authority/trnm-world-command-design.md).
