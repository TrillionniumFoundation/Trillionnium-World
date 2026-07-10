# Trillionnium World Legacy Script Surface

Status: archived route classification as of 2026-07-10.

The historical `check_trillionnium_world_*`, S5/S6 evidence, Android, public
launch, online, OpenRA compatibility and classic renderer scripts are retained
for provenance only. Their presence in `scripts/` does not make them active
game entrypoints and they must not be cited as current gameplay acceptance.

Active game commands are limited to:

- `scripts/run_trnm_first_contact.sh`;
- `scripts/check_trnm_game_product.sh`;
- the focused Cargo commands in
  `docs/development/trillionnium-rpg-rts-closed-loop-v1.md`;
- `playtests/first_contact-closed-loop-acceptance.yaml` and the pending real
  observer record.

`scripts/run_trillionnium_world_bevy_client.sh` remains only as a compatibility
delegator for the existing local systemd unit. Explicit legacy execution lives
at `scripts/archive/run_trnm_legacy_game.sh` and requires `--features legacy`.
