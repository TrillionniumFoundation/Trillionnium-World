# TRNM Platform Workspace

This is the independent chain/node/PoCO platform workspace. It is not a
dependency of the player game and must be built explicitly:

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml --workspace
```

Game release claims cannot cite this workspace as gameplay evidence, and game
iteration does not require platform-wide compilation.
