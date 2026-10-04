# Documentation Policy

Status: active  
Policy version: 1  
Introduced against repository commit: `0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`

## Canonical locations

- `docs/` is the repository-wide documentation root.
- Each independently maintained module owns a code-adjacent `README.md`.
- `trillionnium/docs/` is a specialized technical-document tree for the Rust workspaces and is subordinate to the repository-wide truth hierarchy.
- `docs/archive/` contains historical or superseded material and cannot establish current readiness.

## Truth-source hierarchy

| Question | Canonical source |
| --- | --- |
| Is the current repository release-ready? | `RELEASE_READINESS.md`, cited with the contemporaneous `origin/main` SHA |
| What is the current native-game product state? | `GAME_STATUS.md` |
| Which module owns a behavior? | `PROJECT_BOUNDARY.md`, `docs/modules.json`, and the module README |
| What protocol is normative? | The versioned protocol implementation plus its normative protocol document |
| What remains open? | The active gap-closure board under `docs/release/` |
| What happened in a historical run? | The commit-bound evidence artifact or archived report for that run |

When sources disagree, the more specific active source wins only within its declared scope. No historical PASS, local gate, or subproject result may override a repository-wide NO-GO conclusion.

## Document classes

1. **Normative specification** — required behavior and compatibility rules.
2. **Implementation design** — how current code realizes a specification.
3. **Operations runbook** — diagnosis, mitigation, recovery, and escalation.
4. **User guide** — player, integrator, or operator instructions.
5. **Evidence report** — immutable result bound to commit, artifact, environment, and date.
6. **Architecture decision record** — durable decision, alternatives, and consequences.
7. **Historical/archive** — superseded material retained only for provenance.

Do not combine an evolving implementation plan, normative protocol, user controls, and release verdict in one document unless the scopes are explicitly separated.

## Module documentation minimum

Each module README must state purpose and owner; responsibilities and non-goals; dependencies and public contracts; state model and invariants; error/failure semantics; build/test commands; compatibility/migration policy; security and observability expectations; and known open gaps.

Use [`MODULE_DOCUMENTATION_TEMPLATE.md`](MODULE_DOCUMENTATION_TEMPLATE.md) for new modules.

## Evidence discipline

A release-relevant evidence record must contain exact Git commit and clean/dirty state, build artifact digest, command and environment, runner/OS/architecture/hardware/network profile, UTC timestamp, raw output location, predefined thresholds, evidence scope, and exclusions.

Human evidence must identify real participants through an approved private process. Never fabricate participant names, observations, accessibility results, production traffic, multi-region behavior, or public security posture.

## Link and path discipline

- Canonical documents use repository-relative links.
- Developer-specific absolute workstation paths are forbidden in the canonical surface.
- Canonical links are checked by `scripts/check_docs_quality.py`.
- Archive links need not remain live unless promoted back into active documentation.

## Change discipline

Update documentation when a change affects a public type/message/endpoint/event/config key, state transition/invariant, authority/trust/persistence boundary, migration/compatibility window, operator action/failure mode, or release-evidence claim.

Protocol changes require negative tests and a compatibility statement. Persistence changes require upgrade, rollback, and interrupted-migration behavior.
