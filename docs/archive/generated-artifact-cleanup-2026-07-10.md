# Generated Artifact Cleanup - 2026-07-10

This is the preservation boundary used while cleaning local, ignored build and
acceptance output after the RPG/RTS product workspace split.

## Before cleanup

- repository `target/`: 32 GiB, entirely rebuildable Cargo output;
- `acceptance/`: 28 GiB total;
- `acceptance/S5_native_bevy_device/latest`: 2,281 PPM files totaling
  29,101,746,920 bytes;
- retained S5 artifacts include 926 JSON files, 43 PNG screenshots, 34 XWD
  captures, logs, replay fixtures and small manifests.

## Cleanup policy

- delete the 2,281 generated PPM render dumps; their producing World/Bevy
  scripts are archived and they are not evidence for the current product;
- retain compact JSON/Markdown manifests, PNG screenshots, XWD captures, logs
  and replay fixtures so historical claims remain auditable;
- clean Cargo `target/`, then rebuild and retest the five-crate game product;
- do not mark the pending human five-second or 10-15 minute playtest gates as
  complete.

The removed paths were ignored generated files, not tracked source or human
observations.
