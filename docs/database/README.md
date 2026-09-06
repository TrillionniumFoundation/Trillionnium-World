# Trillionnium World Database Contracts

- `trnm-world-postgresql-contract-v1.md` — supported database identity, migrations, durability, and evidence boundaries;
- `trnm-world-stored-procedure-catalog-v1.md` — procedure classes, privileges, SQLSTATEs, and generated-catalogue target;
- `trnm-world-lock-order-v1.md` — global lock order and transaction-free external-I/O rule.

Migrations are append-only and checksum-bound. Applied SQL is never edited or
rolled back in place. Production restore/PITR evidence is governed by
`../operations/trnm-world-backup-pitr-v1.md`.
