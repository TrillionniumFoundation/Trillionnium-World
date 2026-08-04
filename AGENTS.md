# Project Boundary (binding)

This Git root is **Trillionnium World** (`trillionnium-world`), lane
`game-product`. Before any write, build, commit, branch, remote, or dependency
change, run `bash scripts/project-preflight.sh`.

Stop on a root, project ID, lane, remote, branch, topic, or dependency mismatch.
Use `/home/alex/projects/trillionnium-world`; the old `Trillionnium` path and
the capitalized alias are temporary compatibility links.

This repository owns game-product code: gameplay, campaign, game-server,
simulation, player-facing economy behavior, and match-evidence production. It
does not own Chain consensus/runtime, Hepta evaluation or settlement services,
Nakama infrastructure, or the cross-repository integration harness. The
excluded `trillionnium/crates/platform` tree is legacy material and is not an
active workspace.
