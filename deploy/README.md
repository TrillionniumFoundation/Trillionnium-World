# Deployment Definitions

This tree contains service definitions and deployment-adjacent configuration.

A checked-in unit or manifest proves only deployment shape. Production readiness additionally requires environment-bound installation, health, rollback, backup/recovery, capacity, security, and endurance evidence.

Rules:

- no embedded secrets;
- explicit service user, filesystem permissions, resource limits, and restart policy;
- readiness must test dependencies and authority/fencing state, not just an open port;
- upgrades must document ordering and rollback;
- multi-host or multi-region claims require real cross-host evidence.
