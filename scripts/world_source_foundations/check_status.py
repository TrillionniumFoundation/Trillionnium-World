from __future__ import annotations

from common import ROOT, load

status = load(ROOT / "docs/status/world-source-foundations-v1.json")
claims = status.get("source_claims", {})
for item in ("WORLD-P1-002", "WORLD-P1-003", "WORLD-P1-004"):
    state = claims.get(item, {}).get("state", "")
    if not state or "candidate" not in state:
        raise SystemExit(f"{item} source status must remain candidate-only")
    if claims[item].get("release_effect") != "none":
        raise SystemExit(f"{item} source status must grant no release credit")
authority = status.get("authority", {})
if (
    authority.get("public_online") != "no_go"
    or authority.get("public_player_market") != "disabled"
    or authority.get("commercial_release") != "no_go"
):
    raise SystemExit("source-foundation status widened release posture")
print("source_foundation_truth=passed")
