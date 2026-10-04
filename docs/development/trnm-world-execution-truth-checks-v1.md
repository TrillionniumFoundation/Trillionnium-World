---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P0-009
  - WORLD-P1-009
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Execution truth and qualified checkout checks v1

## Responsibilities and non-goals

These checks prevent two different assertions from being silently substituted:

1. a selected historical execution snapshot versus a current status view;
2. a verified source artifact versus committed source on a local checkout.

Neither check queries GitHub, verifies external approvals, changes a branch,
imports qualified source, signs anything or authorizes production. Passing
checks do not close WORLD-P0-009 or WORLD-P1-001 by themselves. The complete
publication, exact-head qualification and independent review remain separate.

## Selected-snapshot contract

`CURRENT_PLAN.md` carries exactly one line of the form:

```text
<!-- trnm-current-execution-snapshot: docs/status/world-plan-v4-execution-truth-YYYY-MM-DD.json -->
```

It must match the human-readable authoritative snapshot pointer. The snapshot
must have the supported schema and repository identity. Its PR and branch must
match the operative candidate in the root plan. Selection never uses file
mtime, directory enumeration, lexicographic order or the largest version/date.

The checker rejects duplicate JSON keys, non-finite JSON numbers, empty or
missing denominator objects, invalid hashes or IDs, invalid UTC dates,
boolean-as-integer counts, non-boolean closure flags, missing external evidence
classes, unsafe/linked paths and ambiguous or fenced selectors. Production
promotion is unsupported by this version. A schema/renderer change requires
review before a different authorization profile can be represented.

`docs/status/CURRENT.md` is generated deterministically from the selected
snapshot. It includes the snapshot SHA-256 and recorded timestamp, candidate
identity, source-publication flags, Actions observations, independent closure
flags and external evidence rows. It explicitly labels them as recorded
assertions, not live verification. Later explicit root-plan observations keep
their stated scope; the renderer never promotes an old CEX pin into a current
qualified dependency. It does not revive superseded main-protection or template
observations.

## Commands and failure behavior

From the repository root:

```bash
python3 scripts/check-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-qualified-checkout.py
python3 scripts/check-trnm-world-documentation.py
```

Exit zero means the specific check passed; a nonzero result blocks its gate.
The documentation gate invokes the read-only view check and both offline fault
suites. A missing, timed-out or failed child process fails the parent gate.

Default checking does not write source. After an authorized snapshot/pointer
change, a local operator can run:

```bash
python3 scripts/check-trnm-world-execution-truth.py --write
```

This option atomically replaces the view only; it cannot update a snapshot or
closure flag. It is rejected when `CI` or `GITHUB_ACTIONS` is enabled. Do not
unset those flags inside a validation workflow to bypass the rule. CI must
reject stale views rather than regenerate them.

## Qualified checkout contract

The artifact is the existing fixed v13k ZIP. The existing importer verifies all
fixed ZIP/member SHA-256 values, archive member/path/mode safety, reconstructed
Git tree identity, manifest coverage and the exact 73 writes/two deletions.
The checkout verifier then checks both the committed Git HEAD entries and the
working files. Correct uncommitted files cannot hide wrong committed bytes.

```bash
python3 scripts/check-trnm-world-qualified-checkout.py \
  --artifact-zip /absolute/path/to/verified-v13k.zip \
  --expected-head EXACT_40_HEX_CHECKOUT_COMMIT
```

The root must be a World Git checkout with the canonical origin. Every required
write must have the expected blob identity and executable mode in HEAD and the
worktree. Every required deletion must be absent from HEAD and the worktree;
dangling links are not absence. Symlinks, crossed project/origin, unsafe paths,
empty or duplicate records, excessive file sizes and HEAD movement fail closed.
Unrelated governance overlays are allowed and are not modified or certified.

On success the JSON output binds local commit/tree, artifact ZIP/tree and the
number of checked writes/deletions. It always leaves remote branch publication,
exact-head CI and independent review as `not_proven`, and production
authorization as `not_granted`. stdout may be captured as a local evidence
artifact. Failure returns nonzero, never a partial-success JSON document.

## Resource and evidence boundaries

Execution-truth source documents are bounded at 256 KiB. Qualified worktree
files are bounded at 16 MiB. Git output is checked against a 16 MiB budget;
Git subprocesses time out after 60 seconds. Parent documentation checks time
out each child after 90 seconds. The existing importer's archive budgets and
fixed digest checks apply before checkout verification.

The fault suites use synthetic temporary repositories, fixtures and a mocked
HEAD race. They prove checker behavior, not hosted execution. Verification
against the real pinned artifact proves only that exact local source boundary.
It does not prove Rust compilation, database behavior, runner allocation,
server controls, cross-repository compatibility, deployment, custody, endurance,
human/accessibility validation or legal/commercial approval.

## Change and recovery rule

A change to the selector, snapshot schema, rendered fields, artifact pins,
qualified path/mode identities or error behavior must update tests in the same
PR. Do not edit the snapshot to make a failing source or CI gate appear closed.
Investigate the actual mismatched bytes, stale observation or missing evidence,
then obtain the appropriate owner review. Rollback restores the previous
checker and view together; it never grants release credit to older evidence.
