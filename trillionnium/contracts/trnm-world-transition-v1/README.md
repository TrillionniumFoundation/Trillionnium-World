# trnm-world-transition-contract

Zero-dependency Rust reference package for the unsigned deterministic `trnm_world_transition_v1` boundary.

The package owns strict canonical JSON validation, stable request/result types, resource budgets and domain-separated hashes. It deliberately contains no network, database, wall-clock, randomness, signer, wallet or online-authority credential dependency.

Validation:

```bash
cargo fmt --manifest-path Cargo.toml -- --check
cargo test --manifest-path Cargo.toml --locked
cargo clippy --manifest-path Cargo.toml --locked -- -D warnings
```

The independent repository checker consumes the published positive and negative vector files:

```bash
python3 scripts/check-trnm-world-transition-conformance.py
```
