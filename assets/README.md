# Authored Assets

This tree contains authored maps, atlases, audio, and manifests consumed by the native game.

Assets are deterministic product inputs. They do not own gameplay authority, progression, settlement, or protocol behavior. Runtime code must validate identifiers, dimensions, references, and hashes before use.

Requirements:

- retain provenance and license information for every imported or generated asset;
- do not copy proprietary game text, maps, tables, art, audio, or source material;
- prefer stable typed identifiers over display strings;
- treat manifest changes as gameplay changes when they affect pathing, balance, objectives, or economy;
- update replay and balance fixtures when deterministic input changes.

Canonical playable content currently lives under [`first_contact/`](first_contact/).
