# Trillionnium World Evidence Volume Curation - 2026-07-07

Purpose: make the large Native/Bevy acceptance evidence set reviewable before a
handoff without deleting, rewriting, or pretending local artifacts are external
launch evidence.

## Boundary

- Status: local evidence-volume curation plan.
- This is an inventory and handoff plan only.
- Do not delete, compress, move, archive, rewrite, or prune acceptance evidence
  from this plan alone.
- Do not grant public-launch, Android S5 real-device, beta, production-ready UI,
  commercial, multi-node, or public-network credit from host-side evidence
  volume.

## Current Finding

`acceptance/S5_native_bevy_device/latest` is the dominant local evidence-volume
risk. It is much larger than the S6 release-review packet directory because it
contains raw visual/runtime artifacts, especially PPM captures and source
subdirectories for classic RTS review surfaces.

## Curation Rules

1. Preserve `acceptance/S5_native_bevy_device/latest` as the source of truth.
2. Generate a manifest before any future archive or handoff bundle.
3. Keep checksum-bound JSON/Markdown summaries small and reviewer-facing.
4. Separate raw visual evidence from the short reviewer packet.
5. Never treat host-side screenshots, PPMs, or local runner artifacts as Android
   S5 real-device, beta, public-launch, production-ready UI, commercial,
   multi-node, or public-network evidence.
6. Require explicit approval before destructive cleanup or archive movement.

## Handoff Slices

| Slice ID | Contents | Reviewer Use |
| --- | --- | --- |
| `reviewer_summary` | Packet JSON/Markdown, next-plan, runbook, review-slice strategy, readiness summaries. | Start here for status and boundaries. |
| `live_player_screen` | Current runner screenshot/probe and playtest runner status. | Inspect the playable First Contact surface. |
| `representative_visuals` | A small curated list of PPM/PNG screenshots from the largest evidence families. | Review visual/product coverage without opening every raw file. |
| `raw_visual_archive_candidate` | Large PPM/source directories kept in place until explicit archive approval. | Deep audit only; not part of the lightweight reviewer packet. |
| `external_evidence_blockers` | S5/map-pack/beta/commercial/multi-node/public-network missing-real-evidence artifacts. | Confirm public launch remains blocked. |

## Done When

The generated artifact reports total S5 evidence size, file count, large-file
count, top-level heavy entries, top large files, extension counts, curation
rules, and no destructive action performed.
