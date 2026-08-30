# Trillionnium World direct-source tranche

This branch is an internal source-convergence tranche for canonical PR #39.

It materializes `trnm-game-server` as ordinary tracked Rust source, removes the semantic `build.rs` and `src/lib.rs.in` path, removes the Cargo build-script declaration, and preserves transaction-free terminal settlement ownership in `trnm-settlement-worker`.

Acceptance requires a non-empty successful `trnm-world-direct-source / direct-source` run on the current merge candidate. Missing, skipped, cancelled, stale, or failed execution is not success.

This tranche grants no deployed, Nakama-authority, Chain-finality, CEX-custody, public-online, player-market, human-validation, or commercial-release credit.
