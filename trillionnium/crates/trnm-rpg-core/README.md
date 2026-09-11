# `trnm-rpg-core`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Owns the engine-independent RPG/world vocabulary and authored domain rules consumed by the campaign product.

## Responsibilities

- Character attributes, origins, growth paths, skills, equipment modifiers, injuries, and derived stats.
- World graph, room identifiers, NPC schedules/relationships, quests, encounters, markets, and content catalogs.
- Typed deterministic helpers used to build campaign state and battle seeds.

## Non-goals

This crate does not persist saves, settle progression, render UI, accept network input, or run RTS ticks. It must not contain copied proprietary text, maps, tables, art, audio, data, or source code.

## Invariants

- Catalog identifiers are stable, unique, and never inferred from display text.
- World transitions and quest graph movement are validated rather than direct flag writes.
- Content functions are deterministic for the same typed input.
- Combat-affecting equipment has explicit typed modifiers.
- New serialized variants preserve old saves or provide a migration.

Unknown identifiers, invalid transitions, unreachable graph states, missing prerequisites, and malformed content fail before persistent mutation.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-rpg-core --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-rpg-core --all-targets --locked -- -D warnings
```

Additions require identifier uniqueness, graph reachability, catalog exhaustiveness, serialization round-trip, and deterministic-output tests.

## Compatibility and security

Serialized enums and identifiers are persistence-facing. Renames require aliases or migration; removals require save-compatibility evidence. Treat save/content input as hostile, bound collections, and keep display text outside authority decisions.

## References

- [`trillionnium-rpg-rts-closed-loop-v1.md`](../../../docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- [`GAME_STATUS.md`](../../../GAME_STATUS.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)
