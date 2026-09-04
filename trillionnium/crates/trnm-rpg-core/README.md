# trnm-rpg-core

Status: current candidate  
Owner: Trillionnium World RPG domain

## Purpose

This crate owns lightweight, deterministic RPG vocabulary and authored-content definitions used by the playable campaign: character attributes, origins, build paths and titles, equipment and item condition, skills and techniques, encounters, NPC schedules and relationships, quests, world rooms, routes, markets, crafting, and original combat narration.

## Authority and non-goals

It is a pure game-domain library. It does not own campaign persistence, battle settlement, online admission, canonical ordering, wallet balances, presentation, input, or network transport. Historical fixtures and third-party game content must not be imported as current product authority.

## Public contracts

Public domain types and catalogue constants are exported from `src/lib.rs` and `src/content.rs`. Stable identifiers are lowercase, explicit data keys; display strings are never parsed to recover authority or economic semantics. Content consumers must use typed IDs and validation helpers.

## State and invariants

The crate does not persist mutable state. Domain construction and transition helpers must preserve:

- unique room, NPC, quest, item, skill, recipe, faction, and encounter identifiers;
- valid world-graph edges and story locks;
- typed equipment effects for every non-material catalogue item;
- bounded attributes, relationship values, market values, schedules, and encounter actions;
- original content and explicit provenance for every source asset or reference.

## Dependencies and boundaries

The crate may use deterministic collections, serialization, and hashing. It must not import Bevy, SQL, HTTP, environment variables, filesystem state, wall-clock time, or random-number services. `trnm-campaign-core` may depend on this crate; the reverse dependency is forbidden.

## Failure and recovery

Invalid IDs, routes, quest transitions, technique prerequisites, or content shapes return typed rejection without side effects. Recovery is performed by the owning campaign aggregate using a previously valid state; this crate must not silently substitute missing content.

## Testing and evidence

Required tests cover catalogue uniqueness, graph reachability, lock enforcement, quest DAG validity, item/equipment exhaustiveness, market bounds, NPC schedule determinism, encounter state preservation, and content-hash stability. New authored content must include provenance and regression fixtures.

## Compatibility and change control

Renaming or reusing a stable ID is a breaking change. Content additions must declare save compatibility and migration behavior. Attribute formulas, route rules, quest semantics, or catalogue meaning changes require an ADR or versioned ruleset/content revision and deterministic fixture updates.
