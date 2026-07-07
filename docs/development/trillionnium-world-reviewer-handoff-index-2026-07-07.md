# Trillionnium World Reviewer Handoff Index - 2026-07-07

Purpose: give a reviewer a small, checksum-bound entry point into the local
Trillionnium World evidence set without requiring them to open the entire S5
acceptance directory first.

## Boundary

- Status: local reviewer handoff index.
- This is an index over existing local evidence, not a new evidence claim.
- Do not delete, compress, move, archive, rewrite, upload, or publish evidence
  from this index alone.
- Do not grant public-launch, Android S5 real-device, beta, production-ready UI,
  commercial, multi-node, or public-network credit from this local index.

## Source Inputs

- Release packet integrity:
  `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`
- Evidence-volume curation:
  `acceptance/S6_public_launch/latest/trillionnium-world-evidence-volume-curation.json`
- Review-slice strategy:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-slice-strategy.json`
- First Contact observation/runbook artifacts:
  `acceptance/S6_public_launch/latest/first-contact-human-playtest-observation-log.json`
  and `acceptance/S6_public_launch/latest/first-contact-human-playtest-runbook.json`
- Classic playtest handoff packet:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json`

## Index Sections

| Section ID | Contents | Reviewer Use |
| --- | --- | --- |
| `reviewer_summary` | Small JSON/Markdown status, packet, runbook, curation, review-slice, and blocker artifacts. | Read first for state, next action, and no-credit boundaries. |
| `live_player_screen` | Current runner status JSON/probe plus live player-screen PNG. | Inspect the current playable First Contact surface. |
| `representative_visuals` | A short PNG set covering full-game, full-screen, shell/meta, match setup, and HUD surfaces. | Inspect product breadth without loading raw PPM archives. |
| `raw_visual_archive_candidates` | Large PPM evidence files kept in place and checksummed for deep audit. | Prove the large raw evidence remains addressable without making it the first reviewer path. |

## Done When

The generated artifact reports checksums and byte sizes for all indexed files,
keeps raw evidence in place, marks destructive/archive/public actions as false,
and preserves the public-launch/Android S5 blocker boundary.
