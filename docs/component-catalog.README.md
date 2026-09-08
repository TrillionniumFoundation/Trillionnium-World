---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Component catalogue contract

`component-catalog.json` is the complete physical source/application inventory for this repository. It is broader than the active release denominator and exists to prevent retained code from becoming an undocumented or silently active subsystem.

The catalogue classifies every discovered `Cargo.toml` under `trillionnium/` and `contracts/`, plus `web4-frontend/package.json`, with:

- stable component ID and path;
- manifest and component kind;
- lifecycle and release denominator;
- accountable owner;
- canonical documentation;
- executable validation gate.

`check-trnm-world-component-catalog.py` rejects duplicate IDs/paths/manifests, missing paths/docs, unknown lifecycle or kind, incorrect active/legacy denominators, production-authorization promotion, invalid UTF-8, duplicate JSON keys, non-finite values, excessive JSON size/depth/nodes, unclassified manifests and stale catalogue manifests.

A catalogue PASS proves only that all physical manifests have an explicit lifecycle and review surface. It does not prove compilation, correctness, deployment, custody, release eligibility or production authorization.
