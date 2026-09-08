---
status: current-candidate
owner: Trillionnium World RPG domain
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-rpg-core technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A pure Rust domain library containing character attributes and origins, growth paths, equipment and item conditions, skills and techniques, encounters, NPC schedules and relationships, quests, rooms and routes, markets, recipes, factions and original combat narration. Current content is compiled from ordinary source.

## Responsibilities and non-responsibilities

Owns deterministic RPG vocabulary and authored content definitions. It does not own mutable campaign persistence, presentation, online authority, settlement or wallet state.

## Public interfaces and data contracts

Consumers use typed IDs, catalogue constants and deterministic helpers from `src/lib.rs` and `src/content.rs`. Stable identifiers are lowercase data keys. Display text is presentation material and cannot be parsed back into identity, authority or economic meaning. Construction helpers return typed rejection for unknown or invalid content.

## State model and invariants

No durable mutable state is owned. Catalogues require unique IDs; graph edges must reference existing rooms; quest DAGs must be reachable and terminate; equipment effects must be exhaustive; schedules, attributes, relationship values and market values remain bounded. Content provenance and licence status are part of acceptance.

## Dependency direction

May use deterministic collections, serde and hashing. It must not import Bevy, SQL, HTTP, environment variables, filesystem state, wall-clock time or random services. `trnm-campaign-core` may consume this crate; reverse dependency and runtime adapter imports are forbidden.

## Concurrency and cancellation

All APIs are pure or operate on caller-owned values. There are no locks or background tasks. Iteration over maps and sets must use deterministic order where output bytes or hashes are observable. Parallel content validation may only combine results in stable key order.

## Durability and recovery

Content is release-artifact data. Save files store stable IDs and content/rules revisions, not pointers or display strings. Missing or retired content fails closed or uses an explicit versioned migration; silent substitution is forbidden. Provenance records travel with content bundles.

## Failure, retry and idempotency

Unknown IDs, invalid routes, unreachable quest nodes, impossible prerequisites, duplicate catalogue rows and malformed content return bounded domain errors without side effects. Recovery is performed by the campaign aggregate from a previously valid state or a declared migration. Missing content never silently becomes a default reward or route.

## Versioning and migration

Reusing or renaming a stable ID is breaking. Additions declare save compatibility. Formula, route, quest or catalogue semantic changes require a ruleset/content revision and deterministic fixtures. Removal requires an inventory of stored references, migration policy, retirement date and rollback bundle.

## Resource budgets and performance

Catalogue cardinalities, graph depth, route length, quest node count and text size are bounded by content validation. Lookups should be O(log n) or O(1) after deterministic construction; path search must publish worst-case limits. Content hashes exclude nondeterministic source paths and timestamps.

## Observability and security

Pure domain code emits no operational logs. Validation tools report stable content IDs, error codes and source-relative paths, avoiding personal paths. Security concerns include malicious content amplification, identifier spoofing, provenance loss and accidental import of third-party proprietary text or assets.

## Verification traceability

Tests cover uniqueness, graph reachability, story locks, quest DAG validity, equipment exhaustiveness, market bounds, NPC schedule determinism, encounter preservation, content hash stability and migration fixtures. Property tests generate invalid graphs and boundary values; release evidence binds exact source and asset digests.

## Known gaps and change checklist

Move catalogue shape into versioned machine-readable schemas; publish world/quest diagrams and formula reference; add a content-validation CLI; define localization and content-review workflows; and produce stable migration inventories. Every content PR must update provenance, tests, content hash and save-compatibility notes.
