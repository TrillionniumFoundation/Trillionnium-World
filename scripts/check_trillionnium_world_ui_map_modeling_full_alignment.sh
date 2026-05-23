#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY="$OUT_DIR/trnm-world-ui-map-modeling-full-alignment.json"
MARKDOWN="$OUT_DIR/trnm-world-ui-map-modeling-full-alignment.md"
REFRESH="${TRNM_WORLD_FULL_ALIGNMENT_REFRESH:-1}"
REQUIRE_READY=0

for arg in "$@"; do
  case "$arg" in
    --require-ready|--require-full-alignment)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
S4_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
HANDOFF="$S5_DIR/bevy-classic-playtest-handoff-packet.json"
DESKTOP="$S5_DIR/bevy-desktop-real-machine-readiness.json"
MAP_UI_MODELING="$S5_DIR/bevy-classic-rts-map-ui-modeling-readiness.json"
ISOMETRIC_MODELING="$S5_DIR/bevy-classic-isometric-modeling.json"
MODEL_CATALOG="$S5_DIR/bevy-classic-model-catalog.json"
MAP_MODELING="$S4_DIR/map-modeling-gate.json"
PRODUCTION_MAP_PACK="$S4_DIR/production-map-pack-public-evidence.json"
S5_REAL_DEVICE="$S5_DIR/s5-real-device-evidence-validation.json"
PUBLIC_LAUNCH="$OUT_DIR/public-launch-readiness.json"

mkdir -p "$OUT_DIR"

if [[ "$REFRESH" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_public_launch_readiness.sh" >/dev/null
fi

artifact_json() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    printf 'missing full-alignment artifact: %s\n' "$path" >&2
    return 1
  fi
  jq -n \
    --arg label "$label" \
    --arg path "${path#$ROOT/}" \
    --arg sha256 "$(sha256sum "$path" | awk '{print $1}')" \
    --argjson bytes "$(stat -c '%s' "$path")" \
    '{label: $label, path: $path, sha256: $sha256, bytes: $bytes}'
}

ARTIFACTS_JSON="$(
  {
    artifact_json bevy_playtest_handoff_packet "$HANDOFF"
    artifact_json bevy_desktop_real_machine_readiness "$DESKTOP"
    artifact_json bevy_map_ui_modeling_readiness "$MAP_UI_MODELING"
    artifact_json bevy_isometric_modeling "$ISOMETRIC_MODELING"
    artifact_json bevy_model_catalog "$MODEL_CATALOG"
    artifact_json fixture_map_modeling_gate "$MAP_MODELING"
    artifact_json production_map_pack_public_evidence "$PRODUCTION_MAP_PACK"
    artifact_json s5_real_device_evidence_validation "$S5_REAL_DEVICE"
    artifact_json public_launch_readiness "$PUBLIC_LAUNCH"
  } | jq -s .
)"

jq -n \
  --slurpfile handoff "$HANDOFF" \
  --slurpfile desktop "$DESKTOP" \
  --slurpfile mapui "$MAP_UI_MODELING" \
  --slurpfile iso "$ISOMETRIC_MODELING" \
  --slurpfile catalog "$MODEL_CATALOG" \
  --slurpfile mapmodel "$MAP_MODELING" \
  --slurpfile prodmap "$PRODUCTION_MAP_PACK" \
  --slurpfile s5 "$S5_REAL_DEVICE" \
  --slurpfile launch "$PUBLIC_LAUNCH" \
  --argjson artifacts "$ARTIFACTS_JSON" '
  ($handoff[0]) as $handoff |
  ($desktop[0]) as $desktop |
  ($mapui[0]) as $mapui |
  ($iso[0]) as $iso |
  ($catalog[0]) as $catalog |
  ($mapmodel[0]) as $mapmodel |
  ($prodmap[0]) as $prodmap |
  ($s5[0]) as $s5 |
  ($launch[0]) as $launch |
  (
    $handoff.green == true
    and $desktop.green == true
    and $desktop.gates.desktop_before_mobile_gate == true
    and $desktop.gates.android_s5_real_device_not_required_gate == true
    and $mapui.green == true
    and $mapui.visual_gate == true
    and $mapui.command_gate == true
    and $mapui.scroll_gate == true
    and $mapui.camera_gate == true
    and $mapui.preview_gate == true
  ) as $local_ui_aligned |
  (
    $mapmodel.status == "fixture_map_modeling_gate_green_with_public_data_blockers"
    and $mapmodel.fixture_only == true
    and $mapmodel.live_ingestion_enabled == false
    and $mapmodel.runtime_clients_fetch_public_osm_directly == false
    and $mapmodel.public_network_ready == false
    and $mapmodel.gates.all_layers_modeled == true
  ) as $fixture_map_modeling_aligned |
  (
    $iso.green == true
    and $catalog.green == true
    and $mapui.structure_gate == true
    and $mapui.environment_gate == true
    and $mapui.source_policy_gate == true
  ) as $local_modeling_aligned |
  ($prodmap.status == "production_map_pack_public_ready_green") as $production_map_pack_ready |
  ($s5.status == "s5_real_device_evidence_green") as $s5_real_device_ready |
  ($launch.overall_status == "ready_for_public_launch_review") as $public_launch_ready |
  (
    $local_ui_aligned
    and $fixture_map_modeling_aligned
    and $local_modeling_aligned
  ) as $host_side_alignment_green |
  (
    $host_side_alignment_green
    and $production_map_pack_ready
    and $s5_real_device_ready
    and $public_launch_ready
  ) as $full_alignment_green |
  (
    [
      (if $local_ui_aligned then empty else "local_ui_design_alignment" end),
      (if $fixture_map_modeling_aligned then empty else "fixture_map_engine_modeling_alignment" end),
      (if $local_modeling_aligned then empty else "local_modeling_design_alignment" end),
      (if $production_map_pack_ready then empty else "production_map_pack_public_evidence" end),
      (if $s5_real_device_ready then empty else "s5_real_device_evidence" end),
      (if $public_launch_ready then empty else "public_launch_external_evidence" end)
    ] + ($launch.blockers // [])
  ) | unique as $blockers |
  {
    contract_version: "trillionnium_world_ui_map_modeling_full_alignment_v1",
    generated_at: (now | todate),
    source_of_truth: "trillionnium_world_ui_map_modeling_full_alignment_matrix",
    overall_status: (
      if $full_alignment_green then "full_alignment_green"
      elif $host_side_alignment_green then "host_side_ui_map_modeling_aligned_public_evidence_blocked"
      else "blocked_local_ui_map_modeling_alignment"
      end
    ),
    host_side_alignment_green: $host_side_alignment_green,
    full_alignment_green: $full_alignment_green,
    blockers: $blockers,
    alignment_domains: {
      ui_design: {
        local_status: (if $local_ui_aligned then "host_side_human_playtest_handoff_aligned" else "blocked_local_ui_design_alignment" end),
        production_status: (if ($s5_real_device_ready and $public_launch_ready) then "production_ui_alignment_evidence_green" else "blocked_until_s5_real_device_and_external_review_evidence" end),
        source_contracts: {
          handoff_packet: $handoff.contract_version,
          desktop_real_machine_readiness: $desktop.contract_version,
          map_ui_modeling: $mapui.contract_version
        },
        gates: {
          handoff_packet_green: ($handoff.green == true),
          desktop_real_machine_green: ($desktop.green == true),
          desktop_before_mobile_gate: ($desktop.gates.desktop_before_mobile_gate == true),
          android_s5_real_device_not_required_gate: ($desktop.gates.android_s5_real_device_not_required_gate == true),
          map_ui_modeling_green: ($mapui.green == true),
          visual_gate: ($mapui.visual_gate == true),
          command_gate: ($mapui.command_gate == true),
          scroll_gate: ($mapui.scroll_gate == true),
          camera_gate: ($mapui.camera_gate == true),
          preview_gate: ($mapui.preview_gate == true)
        },
        metrics: {
          preview_count: $mapui.preview_count,
          desktop_screenshot_frame_count: $desktop.desktop_evidence.screenshot_frame_count,
          desktop_keyboard_event_count: $desktop.desktop_evidence.keyboard_event_count,
          desktop_release_runner_pid: $desktop.desktop_runtime.release_runner_pid,
          desktop_display: $desktop.desktop_runtime.display,
          title_actions: $handoff.handoff_summary.title_actions,
          runner_service: $handoff.handoff_summary.runner_service,
          runner_main_pid: $handoff.handoff_summary.runner_main_pid,
          visual_fidelity_pixels: $mapui.visual_fidelity_pixels,
          command_affordance_pixels: $mapui.command_affordance_pixels,
          map_camera_pixels: $mapui.map_camera_pixels
        }
      },
      map_engine: {
        local_status: (if $fixture_map_modeling_aligned then "fixture_map_pack_modeling_aligned" else "blocked_fixture_map_modeling_alignment" end),
        production_status: (if $production_map_pack_ready then "production_map_pack_public_evidence_green" else "blocked_missing_production_map_pack_public_evidence" end),
        source_contracts: {
          fixture_map_modeling: $mapmodel.contract_version,
          production_map_pack_public_evidence: $prodmap.contract_version
        },
        gates: {
          fixture_only: ($mapmodel.fixture_only == true),
          live_ingestion_disabled: ($mapmodel.live_ingestion_enabled == false),
          runtime_clients_fetch_public_osm_directly: ($mapmodel.runtime_clients_fetch_public_osm_directly == true),
          public_network_ready: ($mapmodel.public_network_ready == true),
          all_layers_modeled: ($mapmodel.gates.all_layers_modeled == true),
          production_map_pack_ready: $production_map_pack_ready
        },
        metrics: {
          layer_counts: $mapmodel.layer_counts,
          production_map_pack_status: $prodmap.status,
          production_map_pack_blocker_count: (($prodmap.blockers // []) | length),
          production_map_pack_blockers: ($prodmap.blockers // [])
        }
      },
      modeling_design: {
        local_status: (if $local_modeling_aligned then "original_low_spec_classic_rts_modeling_aligned" else "blocked_local_modeling_design_alignment" end),
        production_status: (if ($s5_real_device_ready and $public_launch_ready) then "production_modeling_review_evidence_green" else "blocked_until_s5_render_and_external_review_evidence" end),
        source_contracts: {
          isometric_modeling: $iso.contract_version,
          model_catalog: $catalog.contract_version,
          map_ui_modeling: $mapui.contract_version
        },
        gates: {
          isometric_modeling_green: ($iso.green == true),
          model_catalog_green: ($catalog.green == true),
          structure_gate: ($mapui.structure_gate == true),
          environment_gate: ($mapui.environment_gate == true),
          source_policy_gate: ($mapui.source_policy_gate == true),
          original_art_policy_gate: ($mapui.source_policy_gate == true)
        },
        metrics: {
          isometric_unique_color_count: $iso.unique_color_count,
          isometric_rts_model_entity_count: $iso.rts_model_entity_count,
          model_catalog_unique_color_count: $catalog.unique_color_count,
          modeling_pixels: $mapui.modeling_pixels
        }
      }
    },
    evidence_status: {
      production_map_pack_public: {
        status: $prodmap.status,
        blocker_count: (($prodmap.blockers // []) | length),
        validator_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> ./scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready"
      },
      s5_real_device: {
        status: $s5.status,
        blocker_count: (($s5.blockers // []) | length),
        collector_command: "ANDROID_SERIAL=<device-serial> ./scripts/check_trillionnium_world_s5_device_evidence.sh --require-device",
        validator_command: "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=acceptance/S5_native_bevy_device/latest/s5-device-evidence.json ./scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready"
      },
      public_launch: {
        overall_status: $launch.overall_status,
        blockers: ($launch.blockers // []),
        validator_command: "./scripts/check_trillionnium_world_public_launch_readiness.sh --require-ready"
      }
    },
    run_commands: {
      refresh_full_alignment: "./scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh",
      fast_recheck_existing_artifacts: "TRNM_WORLD_FULL_ALIGNMENT_REFRESH=0 ./scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh",
      refresh_handoff_packet: "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh",
      refresh_map_ui_modeling: "./scripts/check_trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness.sh",
      refresh_fixture_map_modeling: "./scripts/check_trillionnium_world_map_modeling_gate.sh",
      collect_s5_real_device: "ANDROID_SERIAL=<device-serial> ./scripts/check_trillionnium_world_s5_device_evidence.sh --require-device",
      validate_production_map_pack: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> ./scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready",
      validate_public_launch_bundle: "TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH=<real-evidence-bundle.json> ./scripts/check_trillionnium_world_public_launch_evidence_bundle.sh --require-ready"
    },
    artifact_manifest: $artifacts,
    no_credit_boundaries: {
      local_host_side_playtest_is_not_public_launch: true,
      fixture_map_modeling_is_not_production_public_map_pack: true,
      host_side_rendering_is_not_android_s5_real_device_evidence: true,
      no_live_overpass_or_geofabrik_ingestion_performed: true,
      no_public_network_exposure_performed: true,
      proprietary_rts_asset_or_ui_copy_claimed: false
    }
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_ui_map_modeling_full_alignment_v1"
  and .host_side_alignment_green == true
  and .full_alignment_green == false
  and .overall_status == "host_side_ui_map_modeling_aligned_public_evidence_blocked"
  and (.blockers | index("production_map_pack_public_evidence") != null)
  and (.blockers | index("s5_real_device_evidence") != null)
  and .alignment_domains.ui_design.local_status == "host_side_human_playtest_handoff_aligned"
  and .alignment_domains.ui_design.gates.desktop_real_machine_green == true
  and .alignment_domains.ui_design.gates.desktop_before_mobile_gate == true
  and .alignment_domains.ui_design.gates.android_s5_real_device_not_required_gate == true
  and .alignment_domains.map_engine.local_status == "fixture_map_pack_modeling_aligned"
  and .alignment_domains.map_engine.production_status == "blocked_missing_production_map_pack_public_evidence"
  and .alignment_domains.modeling_design.local_status == "original_low_spec_classic_rts_modeling_aligned"
  and .alignment_domains.ui_design.gates.handoff_packet_green == true
  and .alignment_domains.ui_design.gates.map_ui_modeling_green == true
  and .alignment_domains.modeling_design.gates.original_art_policy_gate == true
  and .alignment_domains.map_engine.gates.live_ingestion_disabled == true
  and (.artifact_manifest | length == 9)
' "$SUMMARY" >/dev/null

{
  printf '# TRNM World UI / Map / Modeling Full Alignment\n\n'
  printf -- '- overall_status: `%s`\n' "$(jq -r '.overall_status' "$SUMMARY")"
  printf -- '- host_side_alignment_green: `%s`\n' "$(jq -r '.host_side_alignment_green' "$SUMMARY")"
  printf -- '- full_alignment_green: `%s`\n' "$(jq -r '.full_alignment_green' "$SUMMARY")"
  printf -- '- ui_design: `%s`\n' "$(jq -r '.alignment_domains.ui_design.local_status' "$SUMMARY")"
  printf -- '- map_engine: `%s`; production `%s`\n' \
    "$(jq -r '.alignment_domains.map_engine.local_status' "$SUMMARY")" \
    "$(jq -r '.alignment_domains.map_engine.production_status' "$SUMMARY")"
  printf -- '- modeling_design: `%s`\n\n' "$(jq -r '.alignment_domains.modeling_design.local_status' "$SUMMARY")"
  printf '## Blockers\n\n'
  jq -r '.blockers[] | "- [ ] `" + . + "`"' "$SUMMARY"
  printf '\n## Next Commands\n\n'
  jq -r '.run_commands | to_entries[] | "- `" + .key + "`: `" + .value + "`"' "$SUMMARY"
  printf '\n## Evidence\n\n'
  jq -r '.artifact_manifest[] | "- `" + .label + "`: `" + .path + "` sha256 `" + .sha256 + "`"' "$SUMMARY"
  printf '\n## Boundaries\n\n'
  printf -- '- Host-side Bevy playtest evidence is not public-launch evidence.\n'
  printf -- '- Fixture map modeling is not production public map-pack evidence.\n'
  printf -- '- Host-side rendering is not Android S5 real-device evidence.\n'
  printf -- '- This command performs no live Overpass/Geofabrik ingestion and no public network exposure.\n'
} >"$MARKDOWN"

grep -q 'TRNM World UI / Map / Modeling Full Alignment' "$MARKDOWN"
grep -q 'full_alignment_green: `false`' "$MARKDOWN"
grep -q 'production_map_pack_public_evidence' "$MARKDOWN"
grep -q 's5_real_device_evidence' "$MARKDOWN"

if [[ "$REQUIRE_READY" -eq 1 && "$(jq -r '.full_alignment_green' "$SUMMARY")" != "true" ]]; then
  printf 'TRILLIONNIUM_WORLD_UI_MAP_MODELING_FULL_ALIGNMENT_BLOCKED %s %s\n' "$SUMMARY" "$MARKDOWN" >&2
  exit 1
fi

printf 'TRILLIONNIUM_WORLD_UI_MAP_MODELING_FULL_ALIGNMENT_READY_WITH_BLOCKERS %s %s\n' "$SUMMARY" "$MARKDOWN"
