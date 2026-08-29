---
status: accepted
owner: trillionnium-world
accepted: 2026-08-29
---

# ADR-0003: Reviewable Correctness Source and Non-Self-Modifying CI

## Context

The settlement candidate introduced build-time text transforms over `.rs.in`
templates and workflows that could run formatting or `clippy --fix`, commit the
result, push the candidate branch and publish a verified tag. These mechanisms
make the exact reviewed source differ from the effective compiled or published
candidate and combine author, validator and publisher roles.

## Decision

1. Correctness-critical compiled Rust source must be directly reviewable in the
   repository.
2. Build scripts may generate data, bindings or mechanically derived tables, but
   must not remove, replace or inject authority, settlement, credential,
   persistence or release semantics through text rewriting.
3. CI is read-only with respect to source refs. It may validate, produce logs,
   upload artifacts and publish diagnostics.
4. CI must not run automatic source fixes, create source commits, push candidate
   branches, or create a verified tag for changes it generated.
5. Any machine-suggested patch is applied in a new commit by an accountable
   author and receives ordinary independent review and exact-head validation.
6. Release tags/signatures are produced only from an already reviewed exact
   commit under a separately authorized release workflow.

## Consequences

- Temporary source-transform scripts and self-heal/convergence workflows are
  retired.
- Existing `.rs.in` correctness templates are migrated to normal modules in
  behavior-preserving PRs.
- Formatting/lint failures block the PR rather than silently changing it.
- Evidence can clearly bind reviewed source, compiled source and published
  artifact.

## Validation

- repository search finds no candidate workflow with `contents: write`,
  `cargo clippy --fix`, `git commit`, `git push` or source-tag publication;
- build scripts contain no semantic string replacement over correctness source;
- compiled module files are tracked and match the files reviewed in the PR;
- branch protection and independent review apply to every corrective commit.
