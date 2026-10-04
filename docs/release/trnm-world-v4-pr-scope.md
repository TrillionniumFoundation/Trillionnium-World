# V4 Stacked PR Scope

Base: `fix/world-settlement-gap-closure-v1` at
`1d4dee6d5add45a64f5c138f424e3bdab369ecd4`.

Head: `fix/world-plan-gap-closure-v4`.

Included:

- Plan v4, machine backlog/gap ledger, current documentation hierarchy;
- authority, settlement, determinism, database, security, release, evidence, and runbook contracts;
- strict dependency-free World transition parser/API plus independent vectors;
- settlement runtime v2, migration 0019, quarantine, shutdown, concurrency, and ambiguity controls;
- read-only SHA-pinned World CI and stable required-check contexts;
- status projections that preserve no-go boundaries.

Excluded from this stacked PR:

- Nakama adapter and canonical authority cutover;
- Integration component-lock and cross-repository deployment evidence;
- CEX owner merge and production artifact;
- server-side main ruleset mutation;
- production deployment/PITR/cross-host/public-edge/endurance evidence;
- human, custody, legal, and commercial approvals;
- direct merge of the PR by its author or automation.

Those exclusions remain explicit blockers and cannot be reclassified by source
changes in this repository.
