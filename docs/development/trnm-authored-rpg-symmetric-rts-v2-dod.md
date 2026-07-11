# TRNM Authored RPG + Symmetric RTS v2 Acceptance

Updated: 2026-07-11

This is a bounded software acceptance contract for the July 11 depth pass. It
does not redefine the open-ended historical product vision as a percentage.

## RPG acceptance

- every equippable catalog entry has an explicit typed BattleSeed modifier;
- all fifteen quests expose authored condition graphs, resolution/failure text,
  item/trust/encounter/route gates, retries and approach consequences;
- the world contains twenty connected rooms, including four-room Glass Basin
  and Ashen Fringe regions;
- the three story thresholds apply a persisted Protect / Expose / Accord choice;
- ten NPCs have five relationship stages, moving schedules, pairwise social
  events and bounded persistent memories;
- every sect offers three selectable combat techniques;
- market price is driven by persisted stock and demand; buying, selling,
  crafting, social events and quest outcomes mutate those values.

## RTS acceptance

- four standalone skirmish maps validate and deploy from the title setup;
- both sides use map-resident workers, shared finite resource nodes, cargo
  return, destructible buildings, supply, power, production and research;
- enemy jobs require a surviving worker and sufficient power/supply;
- enemy buildings take damage only from explicit attack authority;
- all twelve unit abilities have unique typed runtime effects;
- dynamic trained enemy units and both sides' structures render with twelve
  unit and ten structure identities backed by project-owned atlas rows;
- a replay export reconstructs and hash-verifies the final simulation snapshot;
- the balance regression executes 24 real-authority samples: four maps × both
  faction assignments × three deterministic seeds, with a bounded pressure
  delta;
- the standalone skirmish E2E issues real commands until a terminal result,
  exports/verifies replay, settles once and rejects duplicate progression.

The five-crate test suite, Clippy `-D warnings`, product-boundary gate, release
build, isolated desktop install and local runtime smoke are the acceptance
evidence. Networking and public-launch evidence remain separate scopes.
