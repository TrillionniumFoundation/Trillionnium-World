---
status: source-candidate
owner: trillionnium-world-operations
work_items:
  - WORLD-P1-004
last_reviewed: 2026-08-31
review_due: 2026-09-14
---

# Trillionnium World Clean-Host Installation Contract v1

## Scope

A supported installation starts from an immutable World source or signed
release artifact and does not require a sibling repository checkout, personal
home directory, mutable branch, or developer workstation state.

## Selector

Production installation requires an exact selector containing the source
commit, source tree, target triple, release profile, component-lock digest, and
artifact SHA-256. Branch names, `latest`, floating toolchains, and unverified
URLs are rejected. The selector is recorded before install and rechecked before
upgrade or rollback.

## Filesystem and identities

- binaries and read-only assets use administrator-owned versioned directories;
- mutable state, logs, runtime sockets, and secrets use separate least-privilege
  directories;
- services run under role-specific non-login identities;
- settlement worker, signer, game server, migration, and observability
  credentials are not interchangeable;
- no install script contains `/home/<developer>`, `/Users/<developer>`, or
  `C:\\Users\\<developer>` paths.

## Lifecycle

Install validates the selector, toolchain/runtime prerequisites, configuration,
directory ownership, migration plan, loopback/default bind posture, and service
units before admission.

Upgrade installs side-by-side, validates checksums and migrations, drains
incompatible ownership, then changes the active immutable selector.

Rollback returns to a previously verified compatible application artifact.
Applied migrations are never edited. Schema-incompatible rollback requires a
separately rehearsed restore or PITR procedure.

Uninstall stops services, removes service registrations and immutable program
files, and preserves or explicitly archives durable player, settlement,
operator, and audit state. Destructive deletion requires a separate audited
operator action.

## Acceptance

Source acceptance requires portable paths, exact selectors, least-privilege
service definitions, and lifecycle scripts or runbooks. Environment acceptance
requires clean-host install, upgrade, rollback, and uninstall rehearsals with
exact artifact, platform, database, and evidence identities. Source completion
alone does not close that environment denominator.
