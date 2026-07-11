# TRNM Original Procedural Audio

`mirror_city_loop.wav` and `signal_battle_loop.wav` are project-owned, clean-room procedural audio assets created for TRNM on 2026-07-11.

They were synthesized locally from simple sine oscillators, amplitude modulation and limiting at 44.1 kHz stereo, matching the current desktop output stream. They contain no sampled performance, third-party melody, proprietary game audio or generated vocals.

- `mirror_city_loop.wav`: 16-second town/title ambience using 110/165/330 Hz layers.
- `signal_battle_loop.wav`: 8-second battle pulse using 55/110/220 Hz layers.

The native Bevy client loops both assets through a dedicated buffered audio thread, switches between them from authoritative campaign mode, and applies the persisted F8 master-volume setting directly to the live players. These loops are a functional original audio baseline, not a claim of final composed soundtrack or final mix.
