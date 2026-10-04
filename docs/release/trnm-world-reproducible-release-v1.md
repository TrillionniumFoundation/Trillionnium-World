---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-004
  - WORLD-P1-006
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Reproducible and Signed Release Contract v1

## Release identity

Every candidate binds:

```text
repository
commit SHA
Git tree SHA
Cargo.lock SHA-256
Rust toolchain/channel/host/LLVM
build profile and target triple
source manifest SHA-256
binary/resource/package SHA-256 and size
SBOM and license inventory SHA-256
workflow/run/job IDs
builder image/runner identity
component lock and deployment profile
signature/attestation identity
```

A tag, branch, PR number, version string, or filename alone is not an immutable
release identity.

## Source conditions

- exact reviewed commit/tree;
- clean worktree/index in the build environment;
- no untracked source/resource input;
- locked dependencies and fixed toolchain;
- only current World workspace members;
- no sibling repository path/environment loading;
- no semantic source rewrite hidden outside the reviewed diff;
- generated artifacts are reproducible from published inputs and fail closed on drift.

Build-time semantic rewriting in the current game-server is tracked migration
debt and must be retired before final production-readiness credit.

## CI conditions

Validation workflows:

- have read-only repository permission;
- do not retain checkout credentials;
- pin Actions to immutable SHA;
- pin runner image, Rust, PostgreSQL, and other critical tools;
- never patch, `clippy --fix`, commit, push, tag, merge, deploy, or self-promote source;
- upload checksummed evidence only;
- expose stable required-check contexts;
- run against the exact PR head.

## Package contents

A complete desktop/server package includes only declared files:

- native client;
- compatibility game server and isolated signer/settlement worker where profile permits;
- assets/maps/audio required by the product;
- portable launcher/desktop metadata;
- runtime requirements and configuration examples;
- source/version/component manifest;
- internal and third-party license/notice inventory;
- SBOM;
- SHA-256 manifest;
- signature/attestation;
- install, uninstall, upgrade, rollback, and verification instructions.

Package verification rejects:

- absolute/parent traversal paths;
- symlink/hardlink/device/FIFO/socket surprises;
- unsafe modes/owners;
- missing/extra payloads;
- digest/size drift;
- executable/script mismatch;
- development credentials or personal paths;
- unverified binary fallback.

## Reproducibility

At least two clean builders reproduce the declared deterministic portions. Any
accepted nondeterminism is documented by path/field/reason and excluded from
security-critical identity. Binary equivalence or normalized reproducibility is
measured, not assumed.

The same source built under a different toolchain, target, feature set, or
component lock is a different release.

## Signatures and attestations

- source tag/commit signing does not replace artifact signing;
- artifact signature binds package digest and release manifest;
- provenance attestation identifies builder, workflow, source, materials, and outputs;
- keys are environment/role scoped and rotated under security policy;
- verification is available offline from retained public material;
- revoked keys remain distinguishable from invalid signatures;
- release promotion verifies all signatures and evidence before selector update.

## Promotion

1. Exact required checks green on the reviewed head.
2. Independent reviewer approves source and limitations.
3. Build produces immutable package, SBOM, licenses, provenance, and signatures.
4. Package is independently verified and installed on clean target hosts.
5. Upgrade/rollback and data compatibility tests pass.
6. Profile-specific runtime/fault/human/security/commercial gates pass.
7. Promotion atomically updates a verified release selector.
8. Old release remains available for the approved rollback window.

The builder workflow does not promote its own artifact.

## Multi-platform

Windows, macOS, and Linux have separate target/signing/install evidence. A Linux
package does not grant Windows/macOS distribution credit. Each platform records:

- supported OS/architecture range;
- code-signing/notarization identity;
- clean-host install/uninstall/upgrade;
- antivirus/quarantine behavior;
- permissions and data paths;
- crash/update recovery;
- accessibility/input/display matrix.

## Acceptance

- toolchain/actions/build inputs fixed;
- SBOM/license/provenance/signature attached and verified;
- clean-host installation needs no source checkout or sibling repository;
- reproducibility measured by independent builders;
- production selector rejects missing/dangling/invalid artifacts;
- multi-platform and deployment-profile evidence remains separately scoped.
