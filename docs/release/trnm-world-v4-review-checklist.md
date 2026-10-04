# Trillionnium World v4 Independent Review Checklist

This checklist is for the stacked PR from `fix/world-plan-gap-closure-v4` into
`fix/world-settlement-gap-closure-v1`. The author or automation that produced the
candidate may not satisfy the independent approval row.

## Exact identity

- [ ] PR head commit and Git tree recorded.
- [ ] Base remains the reviewed settlement gap-closure branch.
- [ ] No unrelated `trillionnium/crates/platform` changes.
- [ ] Diff contains no production/public-market activation.

## Documentation and authority

- [ ] `CURRENT_PLAN.md` points to Plan v4.
- [ ] World/Nakama/Chain/CEX/Integration ownership is unambiguous.
- [ ] Historical Chain/Web4 material is not presented as current World truth.
- [ ] Public online and public player market remain no-go/disabled.

## Transition contract

- [ ] Strict parser rejects all published negative vectors.
- [ ] Positive and request hashes match independent conformance.
- [ ] No player session, private key, canonical root, finality, or settlement surface crosses the World boundary.
- [ ] Stable error and resource budgets are reviewed.

## Settlement

- [ ] Game server cannot perform synchronous remote settlement.
- [ ] Capture, remote execution, and apply are separate.
- [ ] SIGINT/SIGTERM stops admission and drain is bounded.
- [ ] Same-account serialization and unrelated-account concurrency are preserved.
- [ ] Stale leases cannot mutate state.
- [ ] Poison work enters durable quarantine without blocking unrelated work.
- [ ] Malformed successful responses and 409 races recover by exact lookup.
- [ ] Migration 0019 uniqueness/quarantine/operator privileges are reviewed.

## CI and supply chain

- [ ] Exactly one active World workflow exists.
- [ ] Workflow has `contents: read` and no persistent checkout credential.
- [ ] Actions, runner, Rust and PostgreSQL revisions are fixed.
- [ ] Workflow cannot commit, push, tag, merge, or rewrite source.
- [ ] Five exact required contexts pass on the current head.
- [ ] Checksummed artifacts bind head/tree/toolchain/environment.

## Evidence and promotion

- [ ] Source evidence is not described as deployed evidence.
- [ ] CEX/Nakama/Integration external blockers remain explicit.
- [ ] Server-side ruleset is separately observed before governance closure.
- [ ] Human, public-network, cross-host, custody and commercial rows remain external.
- [ ] Reviewer records limitations and final disposition.
