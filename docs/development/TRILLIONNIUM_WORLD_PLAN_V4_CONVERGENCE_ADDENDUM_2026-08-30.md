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

It prevents three recurring failure modes:

1. source being described as remotely verified without a check run;
2. workflow or CODEOWNERS files being described as server-side governance;
3. automated/local evidence being promoted into upstream, deployed, human,
   custody, public-network, legal, or commercial evidence.

Plan V4 remains the ordered development plan. This addendum is its binding
current-state interpretation until superseded by a reviewed later addendum.

## 2. Exact observed repository state

| Fact | Observed value |
| --- | --- |
| Default branch | `main` |
| Observed `main` commit | `efcf0420f6edabc32b7f85332467f25e291cdc63` |
| Parent V4 candidate | `fix/world-plan-gap-closure-v4` |
| Parent candidate head | `0e256625f0c8a64e4079527672b689ee782d6152` |
| Current convergence branch | `fix/world-plan-v4-convergence-2026-08-30` |
| Overlapping V4 candidate | `fix/world-plan-v4-source-gap-closure` |
| Overlapping candidate head | `366becfb33e9992b61611e7aa4924f1fe1b157cc` |
| Check runs on both parent candidate heads | `0` |
| Repository rulesets observed through GitHub API | `0` |
| Read-only Actions probe commit | `2e465c56ba159301796261963204269c9c4cf3c3` |
| Actions runs for the probe commit | `0` |
| Public online | `NO-GO` |
| Public player market | `disabled` |
| Commercial release | `NO-GO` |

The absence of an Actions run is not a passing run. A workflow file is not
proof that Actions are enabled. A CODEOWNERS file is not proof that code-owner
review is required. A documented ruleset is not a server-side ruleset.

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

### `source_open`

Required repository-owned source, schema, test, documentation, or migration is
absent or still violates a source invariant.

### `source_implemented_unverified`

The candidate contains the intended source and source-level tests, but there is
no successful exact-head remote run proving compilation, lint, database
execution, packaging, and negative tests for the same commit.

### `source_verified`

A non-empty exact-head required run passed, the artifact identity is bound to
the same commit/tree, and independent review found no blocker.

### `repository_control_blocked`

Closure requires GitHub repository or organization configuration rather than a
source file, such as enabling Actions or applying a server-side ruleset.

### `blocked_upstream`

Another repository owns the implementation. World may publish contracts and
fixtures but may not mark the upstream implementation complete.

### `environment_evidence_required`

Closure requires a real deployed environment, fault injection, restore,
cross-host topology, endurance interval, public edge, or signed distribution.

### `human_evidence_required`

Closure requires consented human participants. Automation cannot satisfy it.

### `commercial_approval_required`

Closure requires custody, fraud, dispute, support, privacy, legal, finance, or
commercial approval by the accountable owner.

### `closed`

Every required evidence class for the exact claim is complete, current,
reviewed, and bound to the exact promoted component set. Closure is
claim-specific; closing source does not close a release denominator.

## 5. Authority invariants

| Artifact or decision | Sole accountable system |
| --- | --- |
| Authored content and deterministic game rules | World |
| Deterministic state transition and World outcome hash | World |
| Online participant admission | Nakama |
| Canonical total command order | Nakama |
| Online command idempotency and restart recovery | Nakama |
| Canonical archive roots and `MatchCompletedV1` | Nakama |
| Chain ingress, inclusion, consensus and finality | Chain |
| Wallet and ledger settlement and custody | CEX |
| Cross-repository component lock and release matrix | Integration |

The World-local server remains a `world_legacy_local_alpha` compatibility
enclave. It may preserve migration, deterministic, and rollback evidence. It
must not become a second canonical online authority.

## 6. Corrected gap assessment

### 6.1 Deterministic transition contract

The V4 candidate contains a full canonical JSON parser. It enforces:

- object/array roots and complete syntax;
- strict decoded-key ordering and duplicate rejection;
- signed-i64 numbers only;
- minimal escaping and valid UTF-8;
- depth and payload limits;
- exact re-encoding and SHA-256 binding;
- recursive authority-key denial.

This convergence tranche also applies ASCII case folding before authority-key
denial. Mixed-case forms such as `Nakama_Private_Key`,
`MATCH_COMPLETED_V1`, and nested `Chain_App_Hash` fail closed.

Current state: `source_implemented_unverified`.

It becomes `source_verified` only after a non-empty exact-head Rust and
independent conformance run passes.

### 6.2 Settlement capture, execute and apply

The candidate contains the intended transaction-free architecture:

```text
capture transaction
  -> committed durable job
  -> lease-fenced asynchronous signer/CEX execution
  -> durable receipt
  -> exact Campaign apply transaction
```

The source includes stable remote identity, lookup-before-submit, live-lease
fencing, separate remote/application state, account/campaign serialization,
poison-work quarantine, operator replay, and bounded shutdown.

Current state: `source_implemented_unverified`.

It becomes `source_verified` only after mandatory PostgreSQL, Rust, fault-model,
format, and Clippy jobs pass on one exact candidate head.

### 6.3 Directly reviewable compiled source

#### CEX transport — materialized in this tranche

`src/cex.rs` is now the directly compiled and reviewed CEX/signer transport.
The build script no longer reads `src/cex.rs.in`, no longer emits
`trnm_cex_generated.rs`, and the CEX template has been removed.

The direct source contains:

- bounded remote error bodies;
- HTTP 409 ambiguity as retryable;
- malformed success bodies as ambiguous/retryable;
- exact receipt lookup before a later submit;
- no blocking HTTP backend;
- a direct-source regression contract.

This is source progress, not exact-head verification.

#### Game-server and settlement worker — still open

`build.rs` still semantically modifies:

- `src/lib.rs.in`;
- `src/settlement_worker.rs.in`.

It registers migrations, retires runtime paths, rewrites include locations, and
emits the actually compiled code under `OUT_DIR`. Therefore the checked-in
wrappers are still not the complete compiled source for those two modules.

Current state of `WORLD-P0-009`: `source_open`.

Closure requires:

1. materialize `lib.rs` and `settlement_worker.rs` as normal reviewed source;
2. remove semantic source rewriting and `.rs.in` compiled authority;
3. make tests inspect directly compiled files instead of build-script strings;
4. remove `build.rs` if no non-semantic generation remains;
5. pass exact-head full-target and PostgreSQL validation.

A generator may remain only for non-semantic derived artifacts whose inputs and
outputs are both reviewable and drift-checked.

### 6.4 CI execution and repository governance

Read-only SHA-pinned workflows exist, but the repository produced no run for
either parent V4 head and no run for a dedicated push probe.

Current states:

- Actions execution: `repository_control_blocked`;
- exact-head remote evidence: `repository_control_blocked`;
- main ruleset: `repository_control_blocked`;
- independent approval: pending, not synthesizable by source.

Required controls:

- repository and organization Actions policy enabled;
- pull-request workflows allowed for the installed GitHub App;
- protected `main` or an organization ruleset;
- PR-only changes and exact required checks;
- code-owner approval and stale-review dismissal;
- last-push approval by another principal;
- conversation resolution;
- no force-push or branch deletion;
- administrator enforcement or an audited break-glass process.

### 6.5 Nakama and Integration

World cannot close:

- Nakama adapter implementation;
- canonical ordering, idempotency, and recovery;
- Nakama shadow comparison;
- sole `MatchCompletedV1` signing;
- active-match drain and cutover;
- Integration component lock and rollback rehearsal.

Current state: `blocked_upstream`.

### 6.6 Deployment and reliability evidence

These remain `environment_evidence_required`:

- signer and CEX commit followed by response loss;
- SIGTERM/SIGKILL at every worker phase;
- cancellation and shutdown drain;
- database kill-before-ACK and apply rollback;
- PITR/timeline change and old-primary isolation;
- backup restore and receipt retention;
- journal corruption and cross-host fencing;
- multi-client load and settlement backlog;
- a complete 86,400-second isolated endurance run;
- public TLS, WAF/DDoS, and capacity;
- KMS/HSM or equivalent custody boundary;
- Windows, macOS, and Linux signed distribution.

A source fixture, short smoke, or generated template cannot close these rows.

### 6.7 Human, support, and commercial evidence

These remain outside automated closure:

- three independent five-second observers;
- one non-developer unguided 10–15 minute vertical slice;
- keyboard-only, mouse-only, high-contrast, subtitle, low-motion, and viewport
  sessions;
- multiplayer comprehension and recovery;
- moderation, support, and appeal drills;
- custody, anti-fraud, dispute, and chargeback operations;
- privacy, legal, finance, and commercial approval.

Their states remain `human_evidence_required` or
`commercial_approval_required`.

## 7. Ordered convergence loop

### C0 — eliminate duplicate truth

1. Treat `fix/world-plan-gap-closure-v4` as the parent candidate.
2. Apply convergence work only through
   `fix/world-plan-v4-convergence-2026-08-30`.
3. Mark overlapping V4 PRs superseded after unique reviewed changes are
   accounted for.
4. Never merge a stacked descendant before its reviewed base.

Exit: one current candidate and one machine state document.

### C1 — close repository-owned source gaps

1. materialize directly compiled game-server and settlement-worker source;
2. remove remaining semantic build-time rewriting;
3. finish correctness-oriented module extraction without behavior changes;
4. maintain strict canonical JSON and authority-negative vectors;
5. keep remote settlement I/O outside mutable database transactions;
6. keep release and public flags fail closed.

Exit: no `source_open` item owned by World.

### C2 — obtain exact-head evidence

1. enable Actions at repository/organization level;
2. run the read-only exact-head workflow;
3. repair each compile, format, lint, SQL, package, and supply-chain failure
   through a reviewed commit;
4. repeat until every required job passes on one exact head;
5. record run IDs and immutable artifact hashes.

Exit: applicable source items become `source_verified`.

### C3 — enforce server-side governance

Apply and independently query the ruleset. Run a negative rehearsal proving a
boundary-breaking PR cannot merge.

Exit: repository-control rows close.

### C4 — cross-repository authority closure

Nakama and Integration implement and bind the exact World contract, run shadow
comparison, drain legacy matches, cut over canonical admission/completion, and
rehearse rollback.

Exit: no dual canonical authority.

### C5 — deployed reliability and release evidence

Execute complete fault, restore, endurance, topology, public-edge, and
multi-platform matrices against immutable artifacts.

Exit: operational denominators close without borrowing local evidence.

### C6 — human and commercial closure

Collect consented human evidence and accountable commercial approvals.

Exit: only the explicitly approved release denominator may be enabled.

## 8. Definition of done

A gap is closed only when:

1. one exact claim and owner are named;
2. source, schemas, tests, docs, and runbooks agree;
3. negative tests reject invalid or overclaimed evidence;
4. exact commit and tree are recorded;
5. required remote checks are non-empty and successful;
6. applicable artifacts are checksummed and retained;
7. rollback or disablement is rehearsed where applicable;
8. independent review is recorded;
9. evidence class matches claim class;
10. machine state is updated without hiding limitations.

## 9. Stop conditions

Stop promotion immediately if:

- World and Nakama both claim a canonical cursor, root, or completion signature;
- signer/CEX I/O runs under mutable Campaign or match row locks;
- an expired settlement lease can mutate state;
- malformed or ambiguous success causes a blind second submission;
- reviewed and compiled source differ semantically without committed reviewed
  generated output;
- CI can write, commit, push, tag, or promote candidate source;
- required workflows do not execute;
- ruleset enforcement is inferred from files;
- evidence is stale, partial, environment-unbound, or for another commit;
- local/automated evidence is used as human, cross-host, public-network,
  custody, legal, or commercial evidence.

## 10. Release posture

This addendum grants no release credit.

```text
technical source candidate: in progress
exact-head remote verification: blocked by repository control
canonical Nakama online authority: blocked upstream
trusted deployed settlement: environment evidence required
public online: NO-GO
public player market: disabled
commercial release: NO-GO
```
