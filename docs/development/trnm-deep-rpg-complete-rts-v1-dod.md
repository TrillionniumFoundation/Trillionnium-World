# TRNM Deep RPG + Complete RTS v1 Definition of Done

Updated: 2026-07-11

This document defines the finite software scope behind the statement
"deep RPG + complete RTS v1 = 100%". It is a product-scope claim, not a claim
that no more content can ever be authored and not a public/commercial release
claim.

## RPG v1 exit criteria

- one connected twelve-room region with story locks and authoritative route planning;
- three exclusive original sects, three mentors and ten relationship NPCs;
- every NPC has persisted trust, authored low/high-trust dialogue, a daily schedule and an activity;
- fifteen authored quests, each with at least two ordered location steps, a required giver conversation, encounter/settlement gates, rewards and persisted progress;
- seven distinct encounter definitions with encounter-specific combat logs, attack/defend/item/withdraw choices, momentum and a cooldown-gated technique;
- fifteen prerequisite-bound skills whose training costs resources and changes the RTS hero;
- a browsable eighteen-item shop, a browsable four-recipe crafting surface, inventory equipment selection, durability, wear and repair;
- origin, free attributes, mastery, party choice, injuries, supplies, world time, journal and atomic three-slot persistence remain authoritative.

## RTS v1 exit criteria

- six original 40x24 maps: four campaign missions and two repeatable skirmish maps;
- two playable typed factions with six distinct unit archetypes each;
- all ten structure definitions are mapped to authoritative runtime structure kinds and every faction-compatible non-command structure is player-selectable;
- all ten technologies are mapped to authoritative research/upgrade jobs with cost, faction and prerequisite enforcement;
- unit production, structure construction, power, supply, repair, logistics, fog, formations, queues, control groups, stance, patrol, veterancy and adaptive AI remain deterministic and checkpoint-safe;
- skirmish has a real pre-match configuration for map, player/opponent faction, starting resources and Objective/Score/Annihilation victory;
- configuration is persisted, hash-bound into `BattleSeedV1`, executed by the simulation and settled through the normal RPG return path;
- campaign balance and the existing three-to-five-minute authored route remain regression tested.

## Proof and boundary

The five-crate product tests and Clippy gate are the software acceptance proof.
Human observation is a later usability and balance feedback stream; it is not
used to lower or block this software completion percentage. Networking,
public launch, mobile/Windows/macOS packages, endless content volume and a
full commercial soundtrack are separate scopes.
