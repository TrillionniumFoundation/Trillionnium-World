---
status: current
owner: trillionnium-world
as_of: 2026-08-30
applies_to_candidate: fix/world-plan-v4-convergence-2026-08-30
candidate_parent: 0e256625f0c8a64e4079527672b689ee782d6152
main_observed: efcf0420f6edabc32b7f85332467f25e291cdc63
plan: TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md
machine_state: ../status/world-v4-convergence-state-2026-08-30.json
review_due: 2026-09-06
---

# Trillionnium World Plan V4 — convergence addendum

## 1. Purpose

This addendum does not create a competing product plan. It tightens the
execution and evidence semantics of Plan V4 after revalidating the live GitHub
repository on 2026-08-30.

Its purpose is to prevent three recurring failure modes:

1. source that exists being described as remotely verified without a check run;
2. workflow or CODEOWNERS files being described as server-side governance;
3. automated or local evidence being promoted into upstream, deployed, human,
   custody, public-network, legal, or commercial evidence.

Plan V4 remains the ordered development plan. This addendum is the binding
interpretation of current status until superseded by a reviewed later addendum.

## 2. Exact observed repository state

The following observations are facts, not intended target state:

| Fact | Observed value |
| --- | --- |
| Default branch | `main` |
| Observed `main` commit | `efcf0420f6edabc32b7f85332467f25e291cdc63` |
| Canonical V4 candidate before this addendum | `fix/world-plan-gap-closure-v4` |
| Canonical V4 candidate head | `0e256625f0c8a64e4079527672b689ee782d6152` |
| Overlapping V4 candidate | `fix/world-plan-v4-source-gap-closure` |
| Overlapping candidate head | `366becfb33e9992b61611e7aa4924f1fe1b157cc` |
| Check runs on both candidate heads | `0` |
| Repository rulesets observed through GitHub API | `0` |
| Read-only Actions probe commit | `2e465c56ba159301796261963204269c9c4cf3c3` |
| Actions runs for the probe commit | `0` |
| Public online | `NO-GO` |
| Public player market | `disabled` |
| Commercial release | `NO-GO` |

The absence of an Actions run is not a passing run. The presence of a workflow
file is not proof that Actions are enabled. The presence of a CODEOWNERS file is
not proof that code-owner review is required. A documented ruleset is not a
server-side ruleset.

## 3. Current truth hierarchy

When current documents conflict, resolve them in this order:

1. `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json`;
2. accepted ADRs under `docs/adr/`;
3. Plan V4;
4. this addendum;
5. `docs/status/world-v4-convergence-state-2026-08-30.json`;
6. the V4 machine plan and gap ledger;
7. generated status views;
8. operational and implementation documents;
9. historical material.

A lower-ranked document may add detail but may not expand authority, enable a
release denominator, or convert missing evidence into a pass.

## 4. Status vocabulary

Every gap must use exactly one of the following states.

### `source_open`

Required repository-owned source, schema, test, documentation, or migration is
absent or still violates a source invariant.

### `source_implemented_unverified`

The candidate contains the intended source and source-level tests, but there is
no successful exact-head remote run proving compilation, lint, database
execution, packaging, and negative tests for the same commit.

### `source_verified`

A non-empty exact-head required run passed, the artifact identity is bound to
the same commit/tree, and independent review has not found a blocking defect.

### `repository_control_blocked`

Closure requires GitHub repository or organization configuration rather than a
source file. Examples are enabling Actions and applying a server-side ruleset.

### `blocked_upstream`

Another repository owns the implementation. World may publish contracts and
fixtures but may not mark the upstream implementation complete.

### `environment_evidence_required`

Source exists, but closure requires a real deployed environment, fault
injection, restore operation, cross-host topology, endurance interval, or
public edge.

### `human_evidence_required`

Closure requires consented human participants. Automation cannot satisfy it.

### `commercial_approval_required`

Closure requires custody, fraud, dispute, support, privacy, legal, finance, or
commercial approval by the accountable owner.

### `closed`

All required evidence classes for the exact claim are complete, current,
reviewed, and bound to the exact promoted component set. Closure is
claim-specific; closing a source claim does not close a release claim.

## 5. Authority invariants

The authority matrix is unchanged:

| Artifact or decision | Sole accountable system |
| --- | --- |
| Authored content and deterministic game rules | World |
| Deterministic state transition and World outcome hash | World |
| Online participant admission | Nakama |
| Canonical total command order | Nakama |
| Online command idempotency and restart recovery | Nakama |
| Canonical archive roots and `MatchCompletedV1` | Nakama |
| Chain ingress, inclusion, consensus and finality | Chain |
| Wallet and ledger settlement, custody | CEX |
| Cross-repository component lock and release matrix | Integration |

The World-local online server remains a `world_legacy_local_alpha`
compatibility enclave. It may preserve migration, deterministic, and rollback
evidence. It must not become a second canonical online authority.

## 6. Corrected gap assessment

### 6.1 Deterministic transition contract

The V4 candidate contains a full canonical JSON parser rather than the earlier
bracket-and-whitespace approximation. It enforces object/array roots, complete
syntax, strict decoded-key ordering, duplicate rejection, signed-i64 numbers,
minimal escaping, UTF-8, depth and payload limits, exact re-encoding, recursive
authority-key denial, and exact SHA-256 binding.

This convergence tranche additionally requires authority-key denial after ASCII
case folding. Mixed-case forms such as `Nakama_Private_Key` and
`MATCH_COMPLETED_V1` must fail just like their lower-case forms.

Current state: `source_implemented_unverified`.

It becomes `source_verified` only after a non-empty exact-head Rust and
independent conformance run passes.

### 6.2 Settlement capture, execute and apply

The candidate contains the intended transaction-free settlement architecture:

```text
capture transaction
  -> committed durable job
  -> lease-fenced asynchronous signer/CEX execution
  -> durable receipt
  -> exact Campaign apply transaction
```

The source includes stable remote identity, lookup-before-submit, live-lease
fencing, separate remote/application state, account/campaign serialization,
poison-work quarantine, operator replay and bounded shutdown.

Current state: `source_implemented_unverified`.

It becomes `source_verified` only after mandatory PostgreSQL, Rust, fault-model,
format and Clippy jobs pass on the exact candidate head.

### 6.3 Reviewable compiled source

The candidate still uses `build.rs` to read and semantically modify:

- `src/lib.rs.in`;
- `src/settlement_worker.rs.in`;
- `src/cex.rs.in`.

The build script registers migrations, removes or renames runtime paths, changes
remote error classification and emits the actually compiled files under
`OUT_DIR`. Consequently, the small checked-in wrapper is not the complete
compiled source.

Current state: `source_open`.

Closure requires:

1. materialize the generated behavior into normal reviewed Rust source;
2. remove semantic source rewriting from `build.rs`;
3. remove `.rs.in` as a compiled-source authority;
4. ensure tests inspect the directly compiled files rather than build-script
   string markers;
5. pass exact-head full-target and PostgreSQL validation.

A deterministic code generator may remain only for non-semantic derived
artifacts whose inputs and outputs are both reviewable and whose drift is
checked in CI.

### 6.4 CI execution and repository governance

Read-only SHA-pinned workflows exist in source, but the repository produced no
run for either V4 head and no run for a dedicated push probe.

Current states:

- Actions execution: `repository_control_blocked`;
- exact-head remote evidence: `repository_control_blocked`;
- main ruleset: `repository_control_blocked`;
- independent approval: pending, not synthesizable by source.

Required repository controls:

- Actions enabled for the repository and organization policy;
- PR workflows permitted for branches created by the installed GitHub App;
- protected `main` or an organization ruleset;
- PR-only changes;
- exact required checks;
- code-owner approval;
- stale-review dismissal;
- last-push approval by another principal;
- conversation resolution;
- no force-push or branch deletion;
- administrators subject to the rule or an audited break-glass process.

### 6.5 Nakama and Integration

World has published the deterministic transition boundary. World cannot close:

- Nakama adapter implementation;
- canonical ordering and recovery;
- Nakama shadow comparison;
- sole `MatchCompletedV1` signing;
- active-match drain and cutover;
- Integration component lock and rollback rehearsal.

Current state: `blocked_upstream`.

### 6.6 Deployment and reliability evidence

The following remain `environment_evidence_required`:

- signer commit followed by response loss;
- CEX commit followed by response loss;
- SIGTERM and SIGKILL at every worker phase;
- cancellation and shutdown drain;
- database kill-before-ACK;
- apply rollback;
- PITR and timeline transition;
- old-primary isolation;
- backup restore and receipt retention;
- journal corruption;
- cross-host fencing;
- multi-client load and settlement backlog;
- a complete 86,400-second isolated endurance run;
- public TLS, WAF/DDoS and capacity;
- KMS/HSM or equivalent custody boundary;
- Windows, macOS and Linux signed distribution.

A source fixture, short smoke, or generated evidence template cannot close these
rows.

### 6.7 Human, support and commercial evidence

The following remain outside automated closure:

- three independent five-second observers;
- one non-developer unguided 10–15 minute vertical slice;
- keyboard-only, mouse-only, high-contrast, subtitle, low-motion and viewport
  sessions;
- multiplayer comprehension and recovery;
- moderation, support and appeal drills;
- custody, anti-fraud, dispute and chargeback operations;
- privacy, legal, finance and commercial approval.

Their states remain `human_evidence_required` or
`commercial_approval_required`.

## 7. Ordered convergence loop

### C0 — eliminate duplicate truth

1. Treat `fix/world-plan-gap-closure-v4` as the parent candidate.
2. Apply convergence work only through
   `fix/world-plan-v4-convergence-2026-08-30`.
3. Mark overlapping V4 PRs as superseded after unique reviewed changes are
   accounted for.
4. Never merge a stacked descendant before its reviewed base.

Exit: one current candidate and one machine state document.

### C1 — close repository-owned source gaps

1. materialize directly compiled game-server, settlement-worker and CEX source;
2. remove semantic build-time rewriting;
3. finish correctness-oriented module extraction without behavior changes;
4. maintain strict canonical JSON and authority-negative vectors;
5. keep settlement remote I/O outside mutable database transactions;
6. keep release and public flags fail closed.

Exit: no `source_open` item owned by World.

### C2 — obtain exact-head evidence

1. enable Actions at repository/organization level;
2. run the read-only exact-head workflow;
3. repair every actual compile, format, lint, SQL, package and supply-chain
   failure through a new reviewed commit;
4. repeat until all required jobs pass on one exact head;
5. record run IDs and immutable artifact hashes.

Exit: applicable source items become `source_verified`.

### C3 — enforce server-side governance

Apply and independently query the ruleset. Run a negative rehearsal proving a
boundary-breaking PR cannot merge.

Exit: repository-control rows close.

### C4 — cross-repository authority closure

Nakama and Integration implement and bind the exact World contract, perform
shadow comparison, drain legacy matches, cut over canonical admission and
completion, and rehearse rollback.

Exit: no dual canonical authority.

### C5 — deployed reliability and release evidence

Execute the complete fault, restore, endurance, topology, public-edge and
multi-platform matrices against immutable artifacts.

Exit: operational denominators close without borrowing local evidence.

### C6 — human and commercial closure

Collect consented human evidence and accountable commercial approvals.

Exit: only the explicitly approved release denominator may be enabled.

## 8. Definition of done

A gap is closed only when all of the following are true:

1. one exact claim and owner are named;
2. source, schemas, tests, docs and runbooks agree;
3. negative tests reject invalid or overclaimed evidence;
4. exact commit and tree are recorded;
5. required remote checks are non-empty and successful;
6. applicable artifacts are checksummed and retained;
7. rollback or disablement is rehearsed where applicable;
8. independent review is recorded;
9. evidence class matches the claim class;
10. the machine state is updated without hiding limitations.

## 9. Stop conditions

Stop promotion immediately if any of the following is observed:

- World and Nakama both claim a canonical cursor, root or completion signature;
- a signer or CEX call runs under mutable Campaign or match row locks;
- an expired settlement lease can mutate state;
- a malformed or ambiguous success causes a blind second submission;
- the compiled source differs semantically from the reviewed source without a
  committed reviewed generated output;
- CI can write, commit, push, tag or promote candidate source;
- required workflows do not execute;
- ruleset enforcement is inferred from files;
- evidence is stale, partial, environment-unbound or for another commit;
- local/automated evidence is used as human, cross-host, public-network,
  custody, legal or commercial evidence.

## 10. Release posture

This addendum does not grant release credit.

```text
technical source candidate: in progress
exact-head remote verification: blocked by repository control
canonical Nakama online authority: blocked upstream
trusted deployed settlement: environment evidence required
public online: NO-GO
public player market: disabled
commercial release: NO-GO
```
