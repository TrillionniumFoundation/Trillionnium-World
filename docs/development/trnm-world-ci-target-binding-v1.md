---
status: implemented-pending-hosted-validation
owner: trillionnium-world
work_items:
  - WORLD-P0-009
  - WORLD-P1-007
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Event-bound CI target identity and context ownership v1

## Existing defects and repair scope

The 8107a38 input had eight workflow files, while its CI-integrity gate accepted
only one. Five required contexts appeared in both the full product workflow and
the narrower final workflow; the closure context also appeared twice. The final
PostgreSQL job supplied DATABASE_URL, whereas required settlement tests consume
TRNM_SETTLEMENT_TEST_DATABASE_URL. Its prospective-merge job also ran on push
and used text membership rather than exact parent ordering to check PR ancestry.

This repair changes CI wiring, static validation and documentation only. It does
not publish the blocked 73-write/two-deletion source set, fix runtime semantics,
repin the qualified artifact, assert server protection, or enable production.
No previous Cargo test, lint, build, packaging or advisory command is removed.
The presence or success of an identity step is not test execution evidence.

## Single owner of every named check

The integrity gate requires the eight explicitly reviewed workflow filenames and
an exact job-to-context map with twenty unique names. Missing, extra, duplicate,
dynamic or moved context ownership fails. The five canonical
`trnm-world-v4/{docs-governance,transition-contract,settlement-postgres,game-workspace-release,supply-chain}`
checks remain owned by `trnm-world-gap-closure-v4.yml`, which retains full native
workspace, package, database and supply-chain validation. Its push filter now
includes the operative branch; pull requests target main.

The five narrower final jobs use `trnm-world-v4-supplemental/...`. The dedicated
V5 closure workflow retains `trnm-world-v5/closure-contract`; its narrower final
counterpart uses `trnm-world-v5-supplemental/closure-contract`. Required names in
the main-protection contract are unchanged. Declared name ownership is not proof
that GitHub enforces those checks or has scheduled any job.

The integrity inventory is a bounded check of the current ordinary YAML layout,
not a general YAML/shell interpreter. It retains read-only contents, pinned action
revisions, forbidden mutation/privileged-trigger checks, document validation,
and the old candidate schema as a historical input. Current selection continues
to come from CURRENT_PLAN.md and its execution snapshot, never that old record.
`--workflows-only` explicitly produces only static inventory credit; normal CI
uses the full entry point. Child failure or a 180-second timeout is a failure.

## Checkout identity contract

Immediately after checkout, the two primary workflows run:

```bash
python3 scripts/check-trnm-world-ci-target.py --role event --root .
```

The accepted roles are `event`, `head` and `merge`. The full workflow's event role
means the prospective merge for a pull_request event, the pushed head for push,
and a distinctly labelled dispatched head for workflow_dispatch. The seven final
head jobs use head; the final prospective-merge job runs only on pull_request and
uses merge. A push or dispatch cannot produce prospective-merge identity.

The checker reads GITHUB_EVENT_PATH and matches repository name/id, event SHA,
PR number, same-repository head/base, approved World lane, main base and runner
refs. PR event SHA must differ from head/base. The actual checkout SHA must match
its requested role. For a merge, raw commit-header parents must equal the ordered
pair [event base, event head], with no third parent. Commit messages, signatures,
substring matches, a stale merge or a head substituted for a merge cannot satisfy
that check. Local fixture tests do not emulate remote approval.

Committed PROJECT_ID and canonical origin must agree. Tracked or untracked
changes, masked/sparse index entries, raw tracked-byte or executable-mode drift,
a nested checkout directory, missing identity, a linked root/event file,
unsupported privileged event, deleted push or HEAD movement fail closed. Git
inspection uses no network or write command, ignores replacement objects and
external GIT_* redirection, disables fsmonitor and optional index writes, and
never reads a credential value. These checks do not cryptographically authenticate
locally supplied runner variables or protect against arbitrary concurrent file
mutation after the check; hosted logs, final cleanliness and independent review
are still required.

The event input is limited to 2 MiB, unique JSON keys and finite values. Git
inspection is limited to 1 MiB of captured stdout per command and 30 seconds per
command; temporary output is checked before parsing. JSON output records the
role, commit, tree, ordered parents, event digest, run id/attempt and job id. It
always records tests_verified=false, remote_evidence_verified=false and
production_authorization=not_granted. Failure emits no success JSON.

## Workflow integration and runtime prerequisites

All thirteen checkout jobs in the two primary workflows bind target identity
before validation and retain its JSON using the existing SHA-pinned upload-artifact
action, an attempt-specific name and if-no-files-found=error. No repository write
permission is added. A missing identity is not repaired or synthesized by CI.
The historical qualification reconstruction still uses its original source and
toolchain pins. Both workflow environments explicitly set RUSTUP_TOOLCHAIN to
1.98.0 so a checkout's toolchain file cannot silently override the declared lane.
The corrected final Postgres variable retains the mandatory database-test flag.

The checker and tests require Python 3.11+, Git and Bash; only Python standard
library modules are used. The actual Rust, native, PostgreSQL, package and
supply-chain jobs retain their existing runtime dependencies. The new regression
suite uses isolated synthetic local Git objects and event fixtures, never deployed
services, accounts, branch updates or release authority.

```bash
python3 scripts/test-trnm-world-ci-target.py
python3 scripts/test-trnm-world-ci-integrity.py
python3 scripts/check-trnm-world-ci-integrity.py
```

## Compiler advisory and remaining qualification

Rust's official 1.98.1 release (published 2026-09-03) fixes vtable-generation
miscompilation: https://github.com/rust-lang/rust/releases/tag/1.98.1 and
https://github.com/rust-lang/rust/issues/161441. This observation does not establish
that this project's binaries exhibit that bug. It does require explicit review
of the 1.98.0-qualified artifact and a separately identified successor toolchain
validation before release credit. This repair deliberately leaves the immutable
historical artifact pins unchanged; it grants no 1.98.1 execution or compatibility
credit and does not silently transfer historical results to a rebuilt successor.

Source publication, the no-blocking successor manifest, actual Rust/DB/native
execution on final head and merge, live governance, cross-repository dependencies,
and every deployment/custody/human/legal/commercial gate remain open. Zero workflow
runs are not success and do not by themselves identify a scheduler root cause.

## Change and rollback

Changing event support, approved branches, role selection, context ownership or
workflow filenames requires paired positive/negative tests and independent review.
Restore the checker, its tests and workflow wiring together on rollback; do not
remove a failing required check, change protected-main enforcement, or restore a
weaker duplicate context to get a green label. No artifact or source edit can
provide external review or production authorization.


## Raw tracked-byte hardening (2026-09-05 continuation)

A reproduced local counterexample showed why `git status` is not a byte identity
proof: after `git update-index --assume-unchanged` or `--skip-worktree`, a tracked
file could be modified while status remained empty and the previous identity
checker accepted it. This is a checker defect, not evidence of a compromised
hosted run. Git documents these index flags in its official `git-update-index`
and `git-ls-files` manuals.

The identity check now additionally requires a complete stage-zero index equal
to the selected HEAD tree, ordinary `H` entries in `git ls-files -v`, and raw Git
blob identity plus executable mode for every tracked file. A changed worktree
cannot be hidden by a correct HEAD, stat-cache flags, `core.filemode=false`, or a
clean filter that normalizes different bytes to the same index content. The
checker never clears flags, runs filters, refreshes the index, stages content,
or rewrites repository configuration to make these conditions pass.

Tracked inputs are opened relative to a held POSIX root directory descriptor;
each component uses no-follow semantics. Only regular files are supported.
Links, submodules, sparse/conflicted entries, missing files and FIFOs fail
closed. Empty files and UTF-8 filenames including spaces, tabs and newlines
remain supported through NUL-delimited Git inventories. Non-POSIX platforms or
missing no-follow support are unsupported rather than silently downgraded.

The bounds are 50,000 tracked files, 64 MiB per file and 512 MiB in aggregate,
subject also to the existing 1 MiB per Git command inventory limit. Raw hashing
reads 64 KiB chunks. File identity/size/timestamps are checked before and after
reading; index entries and flags are rechecked after the scan, followed by the
untracked-file and HEAD checks. Untracked names are enumerated without
`git status`, with the untracked cache disabled; no clean/smudge filter driver is
invoked during verification. The JSON adds `tracked_files_hashed`,
`tracked_bytes_hashed`, `index_matches_head` and `tracked_blob_bytes_match_head`.
It still grants no test, hosted-execution, independent-review or release credit.

Twenty-four additional real temporary-Git regressions exercise the reproduced
false-clean states, hidden project identity, normalization, executable modes,
staged/worktree disagreement, special paths, resource bounds, file/index races,
and absence of index rewriting. They run inside the existing target regression
suite; there is no new workflow, runtime dependency or permission increase.

This is a point-in-time raw tracked-file check, not an atomic filesystem
snapshot or continuous protection against arbitrary concurrent mutations. It
does not attest ignored/untracked build products, external dependencies, GitHub
runner authenticity, compiler correctness or an actual test run. Existing
untracked-file checks remain. Run the applicable full validation on the exact
final head and prospective merge; missing evidence remains a blocker.
