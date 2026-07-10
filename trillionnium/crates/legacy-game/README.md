# Frozen Legacy Game Workspace

This workspace preserves older World/RTS behavior and evidence tests. It is not
the player product. `trnm-world-bevy` keeps its 167k-line compatibility surface
behind the explicit `legacy` feature; its default build forwards to the
canonical `trnm-first-contact` client.

Only extract a focused mechanic together with a product-owned test. Never
reconnect this workspace wholesale and never promote its older 34x34 map over
`assets/first_contact/maps/first_contact.yaml`.

```bash
cargo test --manifest-path trillionnium/crates/legacy-game/Cargo.toml --workspace
```
