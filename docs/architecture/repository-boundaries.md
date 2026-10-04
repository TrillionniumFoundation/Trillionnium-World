# Repository and Workspace Boundaries

Document class: normative architecture boundary  
Status: active

## Workspace topology

```text
Trillionnium-World/
├── trillionnium/                         # native game workspace
│   └── crates/
│       ├── trnm-*                        # eight game crates
│       └── platform/                     # independent platform workspace
├── contracts/                            # independent Rust contract workspace
├── web4-frontend/                        # independent npm project
├── assets/                               # authored content
├── deploy/                               # service definitions
├── packaging/                            # distribution metadata
└── docs/                                 # repository-wide documentation
```

## Dependency direction

```text
presentation/client
        ↓
campaign + online adapters
        ↓
game-owned protocols
        ↓
deterministic domain cores
```

The native game must not depend directly on the platform workspace. Platform or external-service integration must cross a versioned boundary.

## Authority boundaries

- `trnm-campaign-core` is the persistent RPG progression and settlement authority.
- `trnm-rts-sim` is the deterministic battle authority.
- `trnm-first-contact` is presentation/input, not persistent authority.
- `trnm-game-server` is online session/match authority backed by PostgreSQL and its durability boundary.
- `trnm-economy-protocol` defines intent/receipt semantics but does not own the external ledger.
- `contracts/` currently fixes contract semantics only; it is not the canonical chain runtime.
- `web4-frontend/` is a read-only projection.

## Release isolation

Each workspace has an independent build and evidence denominator. A platform gate cannot satisfy a game gate, a Web4 gate cannot satisfy repository release readiness, and an external-contract unit test cannot satisfy Host ABI or mainnet integration.

## Separation exit criteria

Repository separation is complete only when no forbidden dependency crosses the game/platform boundary; every cross-boundary contract is versioned and compatibility-tested; CI invokes every workspace explicitly; ownership/evidence are independent; and historical documents no longer describe removed workspaces as current implementation.
