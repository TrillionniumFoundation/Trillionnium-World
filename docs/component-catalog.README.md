---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Component catalogue contract

`component-catalog.json` is the complete physical source/application inventory for this repository. It is broader than the active release denominator and exists to prevent retained code from becoming an undocumented or silently active subsystem.

The production verifier walks the repository without following symbolic links, excludes only declared generated/build roots, and discovers every regular `Cargo.toml` and `package.json`. The catalogue must classify that exact discovered manifest set, including active workspaces, isolated authority candidates, protocol/tool contracts such as `trnm-settlement-outbox-contract`, vendored dependencies, historical applications and examples.

Each component records:

- a stable component ID and canonical repository-relative path;
- the manifest located exactly beneath that component path and the component kind;
- lifecycle and release denominator;
- accountable owner;
- component-specific canonical documentation;
- an executable validation gate with direct or workspace-proven traceability;
- source and component-specific test evidence for every non-legacy, non-vendored Rust crate, or bounded build/test/start scripts for Node surfaces.

`scripts/check-trnm-world-component-catalog.py` rejects closed-schema violations, duplicate IDs/paths/manifests, absolute/aliased/parent paths, symbolic-link traversal, manifest/path mismatches, missing or generic documents, unresolved gates, package/component identity drift, unknown lifecycle or kind, lifecycle/denominator contradictions, production-authorization promotion, invalid UTF-8, duplicate JSON keys, non-finite values, excessive JSON size/depth/nodes, unclassified manifests, stale catalogue manifests and unclassified workspace members.

`scripts/test-trnm-world-component-catalog.py` supplies hostile fixtures for schema, path, symbolic-link, discovery, ownership/traceability and lifecycle failures. Both commands must pass on the exact candidate head and its actual prospective merge before catalogue closure receives execution credit.

A catalogue PASS proves only that all physical manifests have an explicit lifecycle and review surface. It does not prove compilation, behavioral correctness, deployment, custody, release eligibility or production authorization.
