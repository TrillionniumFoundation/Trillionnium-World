#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json"
RAW_OUT="$OUT.raw.$$"
TMP_OUT="$OUT.tmp.$$"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
ART_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_art_renderer.rs"
SILHOUETTE_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_silhouette_renderer.rs"
FOCUS_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_focus_renderer.rs"
RADAR_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_radar_renderer.rs"
DATA_SOURCE="$ROOT/trillionnium/crates/trnm-rts-data/src/lib.rs"
RUNTIME_SOURCE="$ROOT/trillionnium/crates/trnm-rts-bevy-runtime/src/lib.rs"
ONLINE_SOURCE="$ROOT/trillionnium/crates/trnm-rts-online/src/lib.rs"
EVIDENCE_SOURCE="$ROOT/trillionnium/crates/trnm-rts-evidence/src/lib.rs"
mkdir -p "$(dirname "$OUT")"

if grep -Fq 'CLASSIC_FIRST_CONTACT_BASIN_ACTORS' "$SOURCE"; then
  echo "[FAIL] First Contact actors must be derived from trnm-rts-data, not a Bevy-local actor table" >&2
  exit 1
fi

if grep -Fq 'enum RtsFirstContactPreviewActorKind' "$SOURCE"; then
  echo "[FAIL] First Contact preview actor kind must live in trnm-rts-data, not Bevy" >&2
  exit 1
fi

if grep -Fq 'fn classic_first_contact_adapter_runtime_handoff_review_input' "$SOURCE"; then
  echo "[FAIL] First Contact offline-adapter handoff review input must live in trnm-rts-online, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let terrain_profiles = first_contact_terrain_profiles();' "$SOURCE"; then
  echo "[FAIL] First Contact terrain profile aggregate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_renderer_projection_gate = renderer_model.renderable_tiles.len()' "$SOURCE"; then
  echo "[FAIL] First Contact renderer projection summary must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if rg -Uq 'let rts_bevy_runtime_map_projection\s*=\s*rts_bevy_runtime::rts_runtime_map_projection\(rts_bevy_runtime::RtsRuntimeMapLayoutInput' "$SOURCE"; then
  echo "[FAIL] First Contact runtime map projection gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let bevy_data_actor_parity_gate = actor_count == bevy_data_actor_templates.len()' "$SOURCE"; then
  echo "[FAIL] First Contact preview actor projection gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_player_screen_layout_gate = player_screen_layout.player_map.map_origin_x == 16' "$SOURCE"; then
  echo "[FAIL] First Contact player-screen layout gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_player_screen_chrome_gate = player_screen_chrome.top_title == "TRNM RTS"' "$SOURCE"; then
  echo "[FAIL] First Contact player-screen chrome gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_player_screen_gate = player_screen_profile.contract_version' "$SOURCE"; then
  echo "[FAIL] First Contact player-screen profile gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_opening_profile_gate = opening_profile.contract_version' "$SOURCE"; then
  echo "[FAIL] First Contact opening profile gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_command_feedback_gate = command_feedback_profile.contract_version' "$SOURCE"; then
  echo "[FAIL] First Contact command-feedback profile gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_player_startup_gate = player_startup_profiles.len() == 4' "$SOURCE"; then
  echo "[FAIL] First Contact player-startup gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_actor_presentation_gate = actor_presentation_profiles.len() >= 13' "$SOURCE"; then
  echo "[FAIL] First Contact actor-presentation gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_visual_telemetry_gate = visual_telemetry_profile.contract_version' "$SOURCE"; then
  echo "[FAIL] First Contact visual-telemetry gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let map_actor_gate = actor_count == 39' "$SOURCE"; then
  echo "[FAIL] First Contact map actor gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rules_gate = unit_count >= 4' "$SOURCE"; then
  echo "[FAIL] First Contact rule summary gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let rts_data_consumer_gate = rts_data_validation_error.is_none()' "$SOURCE"; then
  echo "[FAIL] First Contact data consumer gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

if grep -Fq 'let bevy_map_model_adapter_gate = rts_data_consumer_gate' "$SOURCE"; then
  echo "[FAIL] First Contact map-model adapter gate must live in trnm-rts-evidence, not Bevy" >&2
  exit 1
fi

required_source_lines=(
  'fn classic_first_contact_map_actors_from_rts_data() -> Vec<RtsFirstContactPreviewActor>'
  'first_contact_preview_actors(&first_contact_basin_map())'
  'let map_actors = classic_first_contact_map_actors_from_rts_data();'
  'let actor_template_count = classic_first_contact_map_actors_from_rts_data().len();'
  'fn apply_first_contact_player_screen_application_to_runtime'
  'rts_bevy_runtime::rts_first_contact_player_screen_runtime_application('
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_LABEL_GUARD_CONTRACT'
  'fn classic_first_contact_player_screen_label_guard'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_VISUAL_READABILITY_CONTRACT'
  'fn classic_first_contact_visual_readability_guard'
  'fn classic_draw_first_contact_readability_overlays'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_RADAR_READABILITY_CONTRACT'
  'fn classic_first_contact_radar_readability_guard'
  'fn classic_draw_first_contact_radar_context'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_COMMAND_GRID_READABILITY_CONTRACT'
  'fn classic_first_contact_command_grid_readability_guard'
  'fn classic_first_contact_command_glyph_role'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_BOTTOM_PANEL_READABILITY_CONTRACT'
  'fn classic_first_contact_bottom_panel_readability_guard'
  'fn classic_first_contact_bottom_panel_feedback_label'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT'
  'fn classic_first_contact_silhouette_readability_guard'
  'fn classic_draw_first_contact_silhouette_readability_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ART_READABILITY_CONTRACT'
  'fn classic_first_contact_art_readability_guard'
  'fn classic_first_contact_art_landmark_samples'
  'fn classic_draw_first_contact_art_landmark_detail'
  'fn classic_draw_first_contact_art_readability_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_MOTION_READABILITY_CONTRACT'
  'fn classic_first_contact_motion_readability_guard'
  'first_contact_command_feedback_player_labels(&feedback)'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ATLAS_READABILITY_CONTRACT'
  'fn classic_first_contact_atlas_readability_guard'
  'fn classic_first_contact_atlas_frame_family_samples'
  'fn classic_draw_first_contact_atlas_readability_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_VISUAL_HIERARCHY_CONTRACT'
  'fn classic_first_contact_visual_hierarchy_guard'
  'fn classic_draw_first_contact_visual_hierarchy_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_CENTRAL_CLARITY_CONTRACT'
  'fn classic_first_contact_central_clarity_guard'
  'fn classic_draw_first_contact_central_clarity_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_TERMINAL_LEGIBILITY_CONTRACT'
  'fn classic_first_contact_terminal_legibility_guard'
  'fn classic_draw_first_contact_terminal_legibility_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT'
  'fn classic_first_contact_selection_combat_focus_readability_guard'
  'fn classic_draw_first_contact_selection_combat_focus_layer'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT'
  'fn classic_first_contact_target_callout_guard'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_MARKER_BUDGET_CONTRACT'
  'fn classic_first_contact_atlas_family_lower_lane_tile'
  'fn classic_first_contact_non_focus_owner_identity_color'
  'fn classic_first_contact_owner_identity_color'
  'fn classic_first_contact_base_owner_identity_tiles'
  'fn classic_first_contact_player_visible_tactical_tracks'
  'fn classic_first_contact_player_screen_suppresses_hover_route_path'
  'fn classic_rts_hover_target_preview_path_marker_count'
  'fn classic_first_contact_marker_budget_guard'
  'CLASSIC_FIRST_CONTACT_RUNTIME_CORE_VISIBLE_TILE_Y_MAX'
  'fn classic_first_contact_runtime_core_actor_candidate'
  'fn classic_first_contact_runtime_core_semantic_fixture_actor'
  'fn classic_first_contact_runtime_core_actor_player_visible'
  'fn classic_first_contact_runtime_actor_selection_ring_candidate'
  'fn classic_first_contact_runtime_actor_selection_ring_visible'
  'fn classic_first_contact_runtime_actor_unit_role_accent_candidate'
  'fn classic_first_contact_runtime_actor_unit_role_accent_visible'
  'fn classic_first_contact_runtime_core_visibility'
  'let first_contact_runtime_core_visibility_gate'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SIDEBAR_DENSITY_CONTRACT'
  'fn classic_first_contact_sidebar_density_guard'
  'fn classic_first_contact_palette_state_badge_label'
  'fn classic_first_contact_production_slot_badge_label'
  'fn classic_first_contact_production_status_badge_label'
  'fn classic_first_contact_rendered_production_slot_labels'
  'fn classic_first_contact_rendered_order_queue_labels'
  'let first_contact_player_screen_label_guard_gate'
  'let first_contact_visual_readability_guard_gate'
  'let first_contact_radar_readability_guard_gate'
  'let first_contact_command_grid_readability_guard_gate'
  'let first_contact_sidebar_density_guard_gate'
  'let first_contact_bottom_panel_readability_guard_gate'
  'let first_contact_silhouette_readability_guard_gate'
  'let first_contact_art_readability_guard_gate'
  'let first_contact_motion_readability_guard_gate'
  'let first_contact_atlas_readability_guard_gate'
  'let first_contact_visual_hierarchy_guard_gate'
  'let first_contact_central_clarity_guard_gate'
  'let first_contact_terminal_legibility_guard_gate'
  'let first_contact_selection_combat_focus_guard_gate'
  'let first_contact_target_callout_guard_gate'
  'let first_contact_marker_budget_guard_gate'
  'trnm_rts_evidence::first_contact_bevy_runtime_adapter_evidence()'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_fixture'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_map_model_review'
  'rts_evidence_bevy_runtime_adapter.first_contact_map_model_gate'
  '.first_contact_opening_profile'
  'rts_evidence_bevy_runtime_adapter.first_contact_opening_profile_gate'
  '.first_contact_command_feedback_profile'
  'rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_gate'
  '.first_contact_player_startup_profiles'
  'rts_evidence_bevy_runtime_adapter.first_contact_player_startup_gate'
  '.first_contact_actor_presentation_profiles'
  'rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_gate'
  '.first_contact_visual_telemetry_profile'
  'rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection'
  'rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile'
  'rts_evidence_bevy_runtime_adapter.first_contact_player_screen_layout_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_player_screen_chrome_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_count'
  'rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples'
  'rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection'
  'rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection'
  'rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample'
  'rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample'
  'rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate'
  'rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_review'
  'first_contact_offline_adapter_session_transition_review'
  'rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_review'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE"; then
    echo "[FAIL] missing First Contact RTS data actor derivation source line: $line" >&2
    exit 1
  fi
done

required_art_renderer_source_lines=(
  'fn terrain_samples'
  'fn building_samples'
  'fn landmark_samples'
  'fn draw_terrain_material_depth_detail'
  'fn draw_terrain_detail'
  'fn draw_building_detail'
  'fn draw_landmark_detail'
  'pub(super) fn draw_readability_layer'
)

for line in "${required_art_renderer_source_lines[@]}"; do
  if ! grep -Fq "$line" "$ART_RENDERER_SOURCE"; then
    echo "[FAIL] missing First Contact art renderer source line: $line" >&2
    exit 1
  fi
done

required_silhouette_renderer_source_lines=(
  'fn unit_samples'
  'fn structure_samples'
  'fn terrain_samples'
  'fn draw_unit'
  'fn draw_structure'
  'fn draw_terrain_marker'
  'pub(super) fn draw_readability_layer'
)

for line in "${required_silhouette_renderer_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SILHOUETTE_RENDERER_SOURCE"; then
    echo "[FAIL] missing First Contact silhouette renderer source line: $line" >&2
    exit 1
  fi
done

required_focus_renderer_source_lines=(
  'fn selection_combat_focus_route_tiles'
  'fn route_clearance_tiles'
  'fn draw_route_clearance_corner_cues'
  'fn draw_focus_corner_brackets'
  'fn draw_target_callout'
  'pub(super) fn draw_selection_combat_focus_layer'
)

for line in "${required_focus_renderer_source_lines[@]}"; do
  if ! grep -Fq "$line" "$FOCUS_RENDERER_SOURCE"; then
    echo "[FAIL] missing First Contact focus renderer source line: $line" >&2
    exit 1
  fi
done

required_radar_renderer_source_lines=(
  'fn lane_sample_tiles'
  'fn structure_tiles'
  'fn pressure_tiles'
  'fn objective_tiles'
  'pub(super) fn draw_context'
)

for line in "${required_radar_renderer_source_lines[@]}"; do
  if ! grep -Fq "$line" "$RADAR_RENDERER_SOURCE"; then
    echo "[FAIL] missing First Contact radar renderer source line: $line" >&2
    exit 1
  fi
done

required_evidence_source_lines=(
  'pub struct RtsBevyRuntimeAdapterEvidence'
  'first_contact_player_screen_application: RtsFirstContactPlayerScreenRuntimeApplication'
  'first_contact_offline_adapter_application: RtsOfflineAdapterRuntimeApplication'
  'first_contact_offline_adapter_consumption_review:'
  'first_contact_offline_adapter_session_transition_review:'
  'first_contact_offline_adapter_lobby_ready_review:'
  'first_contact_online_protocol_fixture: trnm_rts_online::RtsOnlineProtocolFixture'
  'first_contact_online_local_handoff: trnm_rts_online::RtsOnlineLocalHandoff'
  'first_contact_online_offline_adapter: trnm_rts_online::RtsOnlineOfflineAdapterSummary'
  'first_contact_online_protocol_gate: bool'
  'first_contact_online_local_handoff_gate: bool'
  'first_contact_online_offline_adapter_gate: bool'
  'pub struct RtsFirstContactMapModelReview'
  'first_contact_map_model_review: RtsFirstContactMapModelReview'
  'first_contact_map_model_gate: bool'
  'first_contact_opening_profile: trnm_rts_data::RtsOpeningLoopProfile'
  'first_contact_opening_profile_gate: bool'
  'first_contact_command_feedback_profile: trnm_rts_data::RtsCommandFeedbackProfile'
  'first_contact_command_feedback_gate: bool'
  'first_contact_player_startup_profiles: Vec<trnm_rts_data::RtsPlayerStartupProfile>'
  'first_contact_player_startup_gate: bool'
  'first_contact_actor_presentation_profiles: Vec<trnm_rts_data::RtsActorPresentationProfile>'
  'first_contact_actor_presentation_gate: bool'
  'first_contact_visual_telemetry_profile:'
  'first_contact_visual_telemetry_gate: bool'
  'pub struct RtsFirstContactPreviewActorProjectionEvidence'
  'first_contact_preview_actor_projection: RtsFirstContactPreviewActorProjectionEvidence'
  'first_contact_preview_actor_projection_gate: bool'
  'first_contact_player_screen_profile: trnm_rts_data::RtsFirstContactPlayerScreenProfile'
  'first_contact_player_screen_layout_gate: bool'
  'first_contact_player_screen_chrome_gate: bool'
  'first_contact_player_screen_profile_gate: bool'
  'pub struct RtsFirstContactTerrainProfileSamples'
  'pub struct RtsFirstContactRendererProjectionEvidence'
  'first_contact_terrain_profile_count: usize'
  'first_contact_terrain_profile_samples: RtsFirstContactTerrainProfileSamples'
  'first_contact_terrain_profile_gate: bool'
  'first_contact_renderer_projection: RtsFirstContactRendererProjectionEvidence'
  'first_contact_renderer_projection_gate: bool'
  'first_contact_runtime_map_projection: RtsRuntimeMapProjection'
  'first_contact_runtime_tile_rect_sample: RtsRuntimeRect'
  'first_contact_runtime_terrain_seed_sample: RtsRuntimeTerrainSeeds'
  'first_contact_runtime_map_projection_gate: bool'
  'pub fn first_contact_bevy_runtime_adapter_evidence'
  'trnm_rts_data::first_contact_player_screen_profile()'
  'first_contact_map_model.validate().err()'
  'first_contact_map_model.summary()'
  'trnm_rts_data::first_contact_opening_loop_profile()'
  'trnm_rts_data::first_contact_command_feedback_profile()'
  'trnm_rts_data::first_contact_player_startup_profiles()'
  'trnm_rts_data::first_contact_actor_presentation_profiles()'
  'trnm_rts_data::first_contact_visual_telemetry_profile()'
  'trnm_rts_data::first_contact_preview_actors(&first_contact_map_model)'
  'trnm_rts_data::first_contact_terrain_profiles()'
  'trnm_rts_data::first_contact_map_renderer_model(&first_contact_map_model)'
  'rts_runtime_map_projection(RtsRuntimeMapLayoutInput'
  'rts_runtime_tile_screen_rect(first_contact_runtime_map_projection, (16, 16))'
  'rts_runtime_terrain_seeds((16, 16))'
  'trnm_rts_online::first_contact_online_protocol_fixture()'
  'trnm_rts_online::rts_online_local_handoff_from_fixture('
  'trnm_rts_online::rts_online_offline_adapter_from_fixture('
  'trnm_rts_online::rts_online_offline_adapter_runtime_handoff_review_input('
  'trnm_rts_online::rts_online_offline_adapter_lobby_ready_review_input('
  'trnm_rts_online::rts_online_offline_adapter_consumption_review_input('
  'rts_first_contact_offline_adapter_runtime_application(&first_contact_runtime_handoff)'
  'rts_first_contact_offline_adapter_consumption_review('
  'rts_first_contact_offline_adapter_session_transition_review('
  'rts_first_contact_offline_adapter_lobby_ready_review('
  'first_contact_offline_adapter_consumption_review: first_contact_consumption_review'
  'first_contact_offline_adapter_session_transition_review:'
  'first_contact_offline_adapter_lobby_ready_review: first_contact_lobby_ready_review'
  'first_contact_online_protocol_fixture: first_contact_online_protocol_fixture'
  'first_contact_online_local_handoff: first_contact_online_local_handoff'
  'first_contact_online_offline_adapter: first_contact_adapter'
  'first_contact_online_protocol_gate'
  'first_contact_online_local_handoff_gate'
  'first_contact_online_offline_adapter_gate'
  'first_contact_map_model_gate'
  'first_contact_opening_profile_gate'
  'first_contact_command_feedback_gate'
  'first_contact_player_startup_gate'
  'first_contact_actor_presentation_gate'
  'first_contact_visual_telemetry_gate'
  'first_contact_preview_actor_projection_gate'
  'first_contact_player_screen_profile_gate'
  'first_contact_terrain_profile_gate'
  'first_contact_renderer_projection_gate'
  'first_contact_runtime_map_projection_gate'
)

for line in "${required_evidence_source_lines[@]}"; do
  if ! grep -Fq "$line" "$EVIDENCE_SOURCE"; then
    echo "[FAIL] missing First Contact RTS evidence aggregate source line: $line" >&2
    exit 1
  fi
done

required_data_source_lines=(
  'pub enum RtsFirstContactPreviewActorKind'
  'pub struct RtsFirstContactPreviewActor'
  'pub fn first_contact_preview_actor_from_map_actor'
  'pub fn first_contact_preview_actors'
  'pub fn first_contact_opening_loop_profile'
  'pub fn first_contact_command_feedback_profile'
  'pub fn first_contact_player_startup_profiles'
  'pub fn first_contact_actor_presentation_profiles'
  'pub fn first_contact_visual_telemetry_profile'
  'openra_preview_rule_id'
)

for line in "${required_data_source_lines[@]}"; do
  if ! grep -Fq "$line" "$DATA_SOURCE"; then
    echo "[FAIL] missing First Contact RTS data preview actor source line: $line" >&2
    exit 1
  fi
done

required_runtime_source_lines=(
  'TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT'
  'pub struct RtsFirstContactPlayerScreenRuntimeApplication'
  'pub fn rts_first_contact_player_screen_runtime_application'
  'TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT'
  'pub struct RtsOfflineAdapterRuntimeApplication'
  'pub fn rts_first_contact_offline_adapter_runtime_application'
  'TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT'
  'pub struct RtsFirstContactOfflineAdapterSessionTransitionReview'
  'pub fn rts_first_contact_offline_adapter_session_transition_review'
  'TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT'
  'pub struct RtsOfflineAdapterLobbyReadyReviewInput'
  'pub struct RtsFirstContactOfflineAdapterLobbyReadyReview'
  'pub fn rts_first_contact_offline_adapter_lobby_ready_review'
  'TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT'
  'pub struct RtsFirstContactOfflineAdapterConsumptionReview'
  'pub fn rts_first_contact_offline_adapter_consumption_review'
  'pub struct RtsRuntimeMapProjection'
  'pub struct RtsRuntimeRect'
  'pub struct RtsRuntimeTerrainSeeds'
  'pub fn rts_runtime_map_projection'
  'pub fn rts_runtime_tile_screen_rect'
  'pub fn rts_runtime_terrain_seeds'
  'pub runtime_application_gate: bool'
)

for line in "${required_runtime_source_lines[@]}"; do
  if ! grep -Fq "$line" "$RUNTIME_SOURCE"; then
    echo "[FAIL] missing First Contact RTS runtime application source line: $line" >&2
    exit 1
  fi
done

required_online_source_lines=(
  'pub fn rts_online_offline_adapter_runtime_handoff_review_input'
  'pub fn rts_online_offline_adapter_lobby_ready_review_input'
  'pub fn rts_online_offline_adapter_consumption_review_input'
  'RtsFirstContactOfflineAdapterConsumptionReviewInput'
  'RtsOfflineAdapterLobbyReadyReviewInput'
  'RtsOfflineAdapterRuntimeHandoffReviewInput'
)

for line in "${required_online_source_lines[@]}"; do
  if ! grep -Fq "$line" "$ONLINE_SOURCE"; then
    echo "[FAIL] missing First Contact RTS online review-input source line: $line" >&2
    exit 1
  fi
done

JQ_FILTER="$(mktemp)"
trap 'rm -f "$RAW_OUT" "$TMP_OUT" "$JQ_FILTER"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-first-contact-basin-spec >"$RAW_OUT"

jq '
  .contract_field_count = ([keys[] | select(endswith("_contract"))] | length)
  | .guard_object_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard$")) and (.value | type == "object"))] | length)
  | .guard_gate_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and (.value | type == "boolean"))] | length)
  | .passed_guard_gate_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and .value == true)] | length)
  | .failed_guard_gate_count = (.guard_gate_count - .passed_guard_gate_count)
  | .top_level_gate_count = ([to_entries[] | select((.key | endswith("_gate")) and (.value | type == "boolean"))] | length)
  | .passed_top_level_gate_count = ([to_entries[] | select((.key | endswith("_gate")) and .value == true)] | length)
  | .failed_top_level_gate_count = (.top_level_gate_count - .passed_top_level_gate_count)
  | .rts_data_map_model_field_count = ((.rts_data_map_model // {}) | keys | length)
  | .rts_data_map_summary_field_count = ((.rts_data_map_summary // {}) | keys | length)
  | .rts_data_map_model_actor_count = ((.rts_data_map_model.actors // []) | length)
  | .rts_data_map_model_player_count = ((.rts_data_map_model.players // []) | length)
  | .rts_data_map_model_rule_count = ((.rts_data_map_model.rules // []) | length)
  | .rts_data_preview_actor_count = ((.rts_data_preview_actors // []) | length)
  | .rts_data_player_startup_profile_count = ((.rts_data_player_startup_profiles // []) | length)
  | .rts_data_actor_presentation_profile_count = ((.rts_data_actor_presentation_profiles // []) | length)
  | .online_protocol_fixture_field_count = ((.rts_online_protocol_fixture // {}) | keys | length)
  | .online_protocol_transport_field_count = ((.rts_online_protocol_fixture.transport // {}) | keys | length)
  | .online_protocol_authority_field_count = ((.rts_online_protocol_fixture.authority // {}) | keys | length)
  | .online_local_handoff_field_count = ((.rts_online_local_handoff // {}) | keys | length)
  | .offline_adapter_field_count = ((.rts_online_offline_adapter // {}) | keys | length)
  | .offline_consumption_field_count = ((.rts_online_offline_adapter_consumption // {}) | keys | length)
  | .offline_session_transition_field_count = ((.rts_online_offline_adapter_session_transition // {}) | keys | length)
  | .offline_lobby_ready_field_count = ((.rts_online_offline_adapter_lobby_ready // {}) | keys | length)
  | .runtime_player_screen_command_queue_count = ((.rts_bevy_runtime_player_screen_application.command_queue // []) | length)
  | .runtime_player_screen_production_queue_count = ((.rts_bevy_runtime_player_screen_application.production_queue // []) | length)
  | .runtime_player_screen_build_queue_count = ((.rts_bevy_runtime_player_screen_application.build_queue // []) | length)
  | .runtime_player_screen_visible_tile_count = ((.rts_bevy_runtime_player_screen_application.visible_tile_ids // []) | length)
  | .runtime_player_screen_fogged_tile_count = ((.rts_bevy_runtime_player_screen_application.fogged_tile_ids // []) | length)
  | .runtime_player_screen_ability_command_count = ((.rts_bevy_runtime_player_screen_application.ability_command_ids // []) | length)
  | .offline_consumption_command_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.command_queue // []) | length)
  | .offline_consumption_production_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.production_queue // []) | length)
  | .offline_consumption_build_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.build_queue // []) | length)
  | .offline_consumption_visible_tile_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.visible_tile_ids // []) | length)
  | .offline_consumption_fogged_tile_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.fogged_tile_ids // []) | length)
  | .offline_consumption_ability_command_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.ability_command_ids // []) | length)
  | .offline_lobby_ready_label_count = ((.rts_online_offline_adapter_lobby_ready.ready_state_labels // []) | length)
' "$RAW_OUT" >"$TMP_OUT"
mv "$TMP_OUT" "$OUT"

cat >"$JQ_FILTER" <<'JQ'
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .contract_field_count == ([keys[] | select(endswith("_contract"))] | length)
  and .guard_object_count == ([to_entries[] | select((.key | test("^first_contact_.*_guard$")) and (.value | type == "object"))] | length)
  and .guard_object_count == 16
  and .guard_gate_count == ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and (.value | type == "boolean"))] | length)
  and .guard_gate_count == 16
  and .passed_guard_gate_count == ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and .value == true)] | length)
  and .passed_guard_gate_count == 16
  and .failed_guard_gate_count == (.guard_gate_count - .passed_guard_gate_count)
  and .failed_guard_gate_count == 0
  and .top_level_gate_count == ([to_entries[] | select((.key | endswith("_gate")) and (.value | type == "boolean"))] | length)
  and .top_level_gate_count == 46
  and .passed_top_level_gate_count == ([to_entries[] | select((.key | endswith("_gate")) and .value == true)] | length)
  and .passed_top_level_gate_count == 46
  and .failed_top_level_gate_count == (.top_level_gate_count - .passed_top_level_gate_count)
  and .failed_top_level_gate_count == 0
  and .map_id == "first_contact_basin"
  and .map_size.width == 34
  and .map_size.height == 34
  and .actor_count == 39
  and .spawn_count == 4
  and .flux_bloom_count == 11
  and .beacon_count == 4
  and .expansion_count == 4
  and .unit_rule_count >= 4
  and .building_rule_count >= 2
  and .map_actor_gate == true
  and .map_topology_gate == true
  and .rules_gate == true
  and .rts_data_contract == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.contract_version == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.map_id == "first_contact_basin"
  and .rts_data_map_model_field_count == (.rts_data_map_model | keys | length)
  and .rts_data_map_model_field_count == 13
  and .rts_data_map_summary_field_count == (.rts_data_map_summary | keys | length)
  and .rts_data_map_summary_field_count == 22
  and (.rts_data_map_model.actors | length) == 39
  and .rts_data_map_model_actor_count == (.rts_data_map_model.actors | length)
  and .rts_data_map_model_actor_count == 39
  and .rts_data_map_model_player_count == (.rts_data_map_model.players | length)
  and .rts_data_map_model_player_count == 6
  and .rts_data_map_model_rule_count == (.rts_data_map_model.rules | length)
  and .rts_data_map_model_rule_count == 19
  and .rts_data_map_summary.actor_count == 39
  and .rts_data_map_summary.source_integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.copied_or_derived == true
  and (.rts_data_source_manifest.source_paths | index("mods/trnm/maps/first-contact-basin/map.yaml") != null)
  and (.rts_data_canonical_sha256 | type == "string" and length == 64)
  and .rts_data_validation_error == null
  and .rts_data_consumer_gate == true
  and .rts_data_map_model_review.map_summary == .rts_data_map_summary
  and .rts_data_map_model_review.unit_rule_count == .unit_rule_count
  and .rts_data_map_model_review.building_rule_count == .building_rule_count
  and .rts_data_map_model_review.data_validation_error == .rts_data_validation_error
  and .rts_data_map_model_review.map_actor_gate == .map_actor_gate
  and .rts_data_map_model_review.map_topology_gate == .map_topology_gate
  and .rts_data_map_model_review.rules_gate == .rules_gate
  and .rts_data_map_model_review.data_consumer_gate == .rts_data_consumer_gate
  and .rts_data_map_model_review.map_model_adapter_gate == .bevy_map_model_adapter_gate
  and .rts_data_map_model_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_map_model_review == .rts_data_map_model_review
  and .rts_evidence_bevy_runtime_adapter.first_contact_map_model_gate == true
  and .rts_data_map_model_gate == .rts_evidence_bevy_runtime_adapter.first_contact_map_model_gate
  and .rts_data_terrain_profile_count == 1156
  and .rts_data_terrain_profile_samples.border.role == "border"
  and .rts_data_terrain_profile_samples.lane.role == "lane"
  and .rts_data_terrain_profile_samples.center.height == 2
  and .rts_data_terrain_profile_samples.base_pad.base_pad == true
  and .rts_data_terrain_profile_samples.resource_zone.resource_zone == true
  and .rts_data_terrain_profile_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_count == 1156
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples.center.height == 2
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples.resource_zone.resource_zone == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_gate == true
  and .rts_data_terrain_profile_count == .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_count
  and .rts_data_terrain_profile_samples == .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples
  and .rts_data_terrain_profile_gate == .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_gate
  and .rts_data_renderer_projection.renderable_tile_count == 1024
  and .rts_data_renderer_projection.lane_tile_count == 240
  and .rts_data_renderer_projection.resource_zone_tile_count == 79
  and .rts_data_renderer_projection.base_pad_tile_count == 144
  and .rts_data_renderer_projection.minimap_anchor_actor_count == 39
  and .rts_data_renderer_projection.resource_actor_tile_count == 11
  and .rts_data_renderer_projection.objective_actor_tile_count == 4
  and .rts_data_renderer_projection.spawn_actor_tile_count == 4
  and (.rts_data_renderer_projection.lane_tile_samples[] | select(.x == 16 and .y == 1))
  and (.rts_data_renderer_projection.resource_actor_tile_samples[] | select(.x == 12 and .y == 16))
  and (.rts_data_renderer_projection.objective_actor_tile_samples[] | select(.x == 16 and .y == 9))
  and (.rts_data_renderer_projection.spawn_actor_tile_samples[] | select(.x == 8 and .y == 8))
  and (.rts_data_renderer_projection.minimap_anchor_actor_samples | index("Actor0") != null)
  and .rts_data_renderer_projection_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.renderable_tile_count == 1024
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.lane_tile_count == 240
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.resource_zone_tile_count == 79
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.base_pad_tile_count == 144
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.minimap_anchor_actor_count == 39
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.resource_actor_tile_count == 11
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.objective_actor_tile_count == 4
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.spawn_actor_tile_count == 4
  and (.rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.minimap_anchor_actor_samples | index("Actor0") != null)
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection_gate == true
  and .rts_data_renderer_projection == .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection
  and .rts_data_renderer_projection_gate == .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection_gate
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_y == 54
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.cell_w == 28
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.cell_h == 14
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_w == 952
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_h == 476
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.x == 464
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.y == 278
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.width == 28
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.height == 14
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample.surface_seed == 12
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample.detail_seed == 20
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate == true
  and .rts_bevy_runtime_map_projection == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection
  and .rts_bevy_runtime_tile_rect_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample
  and .rts_bevy_runtime_terrain_seed_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample
  and .rts_bevy_runtime_map_projection_gate == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate
  and .rts_data_opening_profile.contract_version == "trnm_rts_data_first_contact_opening_profile_v1"
  and .rts_data_opening_profile.map_id == "first_contact_basin"
  and .rts_data_opening_profile.active_beacon_tile.x == 16
  and .rts_data_opening_profile.active_beacon_tile.y == 9
  and .rts_data_opening_profile.active_relay_tile.x == 11
  and .rts_data_opening_profile.active_relay_tile.y == 8
  and .rts_data_command_feedback_profile.contract_version == "trnm_rts_data_first_contact_command_feedback_v1"
  and .rts_data_command_feedback_profile.target_tile.x == 16
  and .rts_data_command_feedback_profile.target_tile.y == 9
  and .rts_data_command_feedback_profile.blocked_tile.x == 15
  and .rts_data_command_feedback_profile.blocked_tile.y == 16
  and .rts_data_opening_profile_gate == true
  and .rts_data_command_feedback_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.contract_version == "trnm_rts_data_first_contact_opening_profile_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.map_id == "first_contact_basin"
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.active_beacon_tile.x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.active_beacon_tile.y == 9
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.active_relay_tile.x == 11
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile.active_relay_tile.y == 8
  and .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile.contract_version == "trnm_rts_data_first_contact_command_feedback_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile.target_tile.x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile.target_tile.y == 9
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile.blocked_tile.x == 15
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile.blocked_tile.y == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_gate == true
  and .rts_data_opening_profile == .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile
  and .rts_data_opening_profile_gate == .rts_evidence_bevy_runtime_adapter.first_contact_opening_profile_gate
  and .rts_data_command_feedback_profile == .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_profile
  and .rts_data_command_feedback_gate == .rts_evidence_bevy_runtime_adapter.first_contact_command_feedback_gate
  and .rts_data_preview_actor_contract == "trnm_rts_data_first_contact_preview_actor_v1"
  and .rts_data_preview_actor_projection.actor_count == 39
  and .rts_data_preview_actor_projection.spawn_count == 4
  and .rts_data_preview_actor_projection.flux_bloom_count == 11
  and .rts_data_preview_actor_projection.beacon_count == 4
  and .rts_data_preview_actor_projection.expansion_count == 4
  and .rts_data_preview_actor_projection_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.actor_count == 39
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.spawn_count == 4
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.flux_bloom_count == 11
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.beacon_count == 4
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.expansion_count == 4
  and (.rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor0" and .kind == "spawn" and .owner == "Multi0" and .tile.x == 8 and .tile.y == 8 and .source_rule_id == "mpspawn" and .openra_preview_rule_id == "trnm.map.detail"))
  and (.rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor2" and .kind == "flux_bloom" and .tile.x == 12 and .tile.y == 16 and .source_rule_id == "trnm.flux.bloom" and .openra_preview_rule_id == "trnm.flux.bloom"))
  and (.rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor5" and .kind == "spawn" and .owner == "Multi2" and .tile.x == 25 and .tile.y == 8))
  and .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection_gate == true
  and .rts_data_preview_actor_projection == .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection
  and .rts_data_preview_actor_projection_gate == .rts_evidence_bevy_runtime_adapter.first_contact_preview_actor_projection_gate
  and (.rts_data_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor0" and .kind == "spawn" and .owner == "Multi0" and .tile.x == 8 and .tile.y == 8 and .source_rule_id == "mpspawn" and .openra_preview_rule_id == "trnm.map.detail"))
  and (.rts_data_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor2" and .kind == "flux_bloom" and .tile.x == 12 and .tile.y == 16 and .source_rule_id == "trnm.flux.bloom" and .openra_preview_rule_id == "trnm.flux.bloom"))
  and (.rts_data_preview_actor_projection.actor_samples[] | select(.source_actor_id == "Actor5" and .kind == "spawn" and .owner == "Multi2" and .tile.x == 25 and .tile.y == 8))
  and (.rts_data_preview_actors | length) == 39
  and .rts_data_preview_actor_count == (.rts_data_preview_actors | length)
  and .rts_data_preview_actor_count == 39
  and (.rts_data_preview_actors[] | select(.source_actor_id == "Actor15" and .kind == "beacon" and .tile.x == 16 and .tile.y == 9 and .openra_preview_rule_id == "trnm.flux.beacon"))
  and (.rts_data_preview_actors[] | select(.source_actor_id == "Actor35" and .kind == "expansion_marker" and .tile.x == 11 and .tile.y == 8 and .source_rule_id == "trnm.expansion.marker" and .openra_preview_rule_id == "trnm.map.detail"))
  and (.rts_data_player_startup_profiles | length) == 4
  and .rts_data_player_startup_profile_count == (.rts_data_player_startup_profiles | length)
  and .rts_data_player_startup_profile_count == 4
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi0" and .faction == "horizon" and .spawn_tile.x == 8 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi1" and .faction == "forge" and .spawn_tile.x == 25 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi2" and .faction == "horizon" and .spawn_tile.x == 25 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi3" and .faction == "forge" and .spawn_tile.x == 8 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and .rts_data_player_startup_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_startup_profiles == .rts_data_player_startup_profiles
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_startup_gate == true
  and .rts_data_player_startup_gate == .rts_evidence_bevy_runtime_adapter.first_contact_player_startup_gate
  and .rts_data_actor_presentation_contract == "trnm_rts_data_first_contact_actor_presentation_v1"
  and .rts_data_actor_glyph_contract == "trnm_rts_data_first_contact_actor_glyph_v1"
  and (.rts_data_actor_presentation_profiles | length) >= 13
  and .rts_data_actor_presentation_profile_count == (.rts_data_actor_presentation_profiles | length)
  and .rts_data_actor_presentation_profile_count == 14
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "mpspawn" and .glyph.body == "spawn_pad" and .glyph.accent == "owner_stripe" and .glyph.footprint_width_cells == 3))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.worker" and .color_role == "worker" and .glyph_role == "worker" and .structure == false and .selectable == true and .glyph.body == "unit" and .glyph.accent == "worker_cargo" and .glyph.selection_ring == true))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.command.core" and .color_role == "command_core" and .glyph_role == "command_core" and .structure == true and .health_bar_width >= 32 and .glyph.body == "structure" and .glyph.accent == "command_spire" and .glyph.footprint_width_cells == 2))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.flux.beacon" and .color_role == "objective" and .glyph_role == "beacon" and .structure == true and .glyph.body == "objective_beacon" and .glyph.accent == "beacon_core"))
  and .rts_data_actor_presentation_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_profiles == .rts_data_actor_presentation_profiles
  and .rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_gate == true
  and .rts_data_actor_presentation_gate == .rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_gate
  and .rts_data_visual_telemetry_contract == "trnm_rts_data_first_contact_visual_telemetry_v1"
  and .rts_data_visual_telemetry_profile.contract_version == "trnm_rts_data_first_contact_visual_telemetry_v1"
  and .rts_data_visual_telemetry_profile.map_id == "first_contact_basin"
  and (.rts_data_visual_telemetry_profile.unit_statuses | length) == 4
  and (.rts_data_visual_telemetry_profile.tactical_tracks | length) == 6
  and (.rts_data_visual_telemetry_profile.unit_statuses[] | select(.tile.x == 8 and .tile.y == 8 and .role_badge == "W" and .role_color == "health" and .health_percent == 82 and .shield_percent == 44))
  and (.rts_data_visual_telemetry_profile.tactical_tracks[] | select(.from_tile.x == 11 and .from_tile.y == 8 and .to_tile.x == 16 and .to_tile.y == 9 and .color_role == "action_trail"))
  and .rts_data_visual_telemetry_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_profile == .rts_data_visual_telemetry_profile
  and .rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_gate == true
  and .rts_data_visual_telemetry_gate == .rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_gate
  and .rts_data_player_screen_contract == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_data_player_screen_profile.contract_version == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_data_player_screen_profile.map_id == "first_contact_basin"
  and .rts_data_player_screen_profile.room_id == "first-contact-basin"
  and .rts_data_player_screen_profile.layout.player_map.map_origin_x == 16
  and .rts_data_player_screen_profile.layout.player_map.map_origin_y == 54
  and .rts_data_player_screen_profile.layout.player_map.right_reserved_px == 292
  and .rts_data_player_screen_profile.layout.player_map.bottom_reserved_px == 158
  and .rts_data_player_screen_profile.layout.player_map.cell_width.min == 12
  and .rts_data_player_screen_profile.layout.player_map.cell_width.max == 28
  and .rts_data_player_screen_profile.layout.player_map.cell_height.min == 8
  and .rts_data_player_screen_profile.layout.player_map.cell_height.max == 15
  and .rts_data_player_screen_layout_profile.player_map.map_origin_x == 16
  and .rts_data_player_screen_layout_profile.spec_map.map_origin_x == 24
  and .rts_data_player_screen_layout_profile.spec_map.map_origin_y == 110
  and .rts_data_player_screen_layout_profile.spec_map.right_reserved_px == 266
  and .rts_data_player_screen_layout_profile.spec_map.cell_width.min == 10
  and .rts_data_player_screen_layout_profile.spec_map.cell_width.max == 22
  and .rts_data_player_screen_layout_profile.map_outer_padding_px == 8
  and .rts_data_player_screen_layout_profile.map_inner_padding_px == 4
  and .rts_data_player_screen_layout_gate == true
  and .rts_data_player_screen_profile.chrome.top_title == "TRNM RTS"
  and .rts_data_player_screen_profile.chrome.skirmish_status_label == "LOCAL SKIRMISH  OWNED ASSETS"
  and .rts_data_player_screen_profile.chrome.tactical_view_title == "TACTICAL VIEW"
  and .rts_data_player_screen_profile.chrome.tactical_view_camera_prefix == "CAM"
  and .rts_data_player_screen_profile.chrome.tactical_view_zoom_prefix == "Z"
  and .rts_data_player_screen_profile.chrome.tactical_view_default_camera_tile.x == 16
  and .rts_data_player_screen_profile.chrome.tactical_view_default_camera_tile.y == 16
  and .rts_data_player_screen_profile.chrome.tactical_view_status_fallback == "GROUP 1  ATTACK QUEUED"
  and .rts_data_player_screen_profile.chrome.tactical_view_status_max_chars == 40
  and (.rts_data_player_screen_profile.chrome.resource_readouts | length) == 4
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "credits" and .label == "CREDITS"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "power" and .label == "POWER"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "supply" and .label == "SUPPLY"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "visibility" and .label == "VISION"))
  and .rts_data_player_screen_profile.chrome.radar_title == "RADAR"
  and .rts_data_player_screen_profile.chrome.production_title == "PRODUCTION"
  and .rts_data_player_screen_profile.chrome.build_palette_title == "BUILD PALETTE"
  and .rts_data_player_screen_profile.chrome.production_empty_label == "ready"
  and .rts_data_player_screen_profile.chrome.production_slot_visible_count == 4
  and .rts_data_player_screen_profile.chrome.production_slot_column_count == 2
  and (.rts_data_player_screen_profile.chrome.build_palette_slots | length) == 8
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "POWER" and .queue_id == "build:power_node@5,3"))
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "TRAIN" and .queue_id == "build:training_hall@4,3"))
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "SIGNAL" and .queue_id == "upgrade:signal_blade"))
  and .rts_data_player_screen_profile.chrome.build_palette_visible_count == 8
  and .rts_data_player_screen_profile.chrome.build_palette_column_count == 4
  and .rts_data_player_screen_profile.chrome.tactics_title == "TACTICS"
  and (.rts_data_player_screen_profile.chrome.tactics_rows | length) == 5
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "order" and .label == "ORDER" and .max_value_chars == 20))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "target" and .label == "TARGET" and .empty_label == "NONE"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "camera" and .label == "CAM" and .empty_label == "-"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "queue" and .label == "QUEUE"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "build" and .label == "BUILD" and .empty_label == "NONE"))
  and .rts_data_player_screen_chrome_profile.selection_panel_title == "SELECTION"
  and .rts_data_player_screen_chrome_profile.selection_card_visible_count == 5
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | length) == 5
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | index("actor_player_idle_south") != null)
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | index("prop_banner") != null)
  and .rts_data_player_screen_chrome_profile.selection_card_health_fallback_percent == 80
  and .rts_data_player_screen_chrome_profile.selection_feedback_label_max_chars == 62
  and .rts_data_player_screen_chrome_profile.command_panel_title == "COMMANDS"
  and .rts_data_player_screen_chrome_profile.command_grid_slot_count == 12
  and .rts_data_player_screen_chrome_profile.command_grid_column_count == 6
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | length) == 6
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("relay") != null)
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("signal") != null)
  and .rts_data_player_screen_chrome_profile.command_slot_fallback_id == "hold"
  and .rts_data_player_screen_chrome_profile.order_queue_title == "ORDER QUEUE"
  and .rts_data_player_screen_chrome_profile.order_queue_empty_label == "NO ORDERS"
  and .rts_data_player_screen_chrome_profile.order_queue_visible_count == 5
  and .rts_data_player_screen_chrome_profile.order_queue_label_max_chars == 32
  and .first_contact_player_screen_label_guard_contract == "trillionnium_world_bevy_classic_rts_first_contact_label_guard_v1"
  and .first_contact_player_screen_label_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_label_guard_v1"
  and .first_contact_player_screen_label_guard.green == true
  and .first_contact_player_screen_label_guard.resource_labels == ["CREDITS","POWER","SUPPLY","VISION"]
  and .first_contact_player_screen_label_guard.production_slot_labels == ["GUARD","WORKER","SIGNAL","TRAINING"]
  and .first_contact_player_screen_label_guard.production_slot_badge_labels == ["GRD","WRK","SIG","TRN"]
  and (.first_contact_player_screen_label_guard.production_slot_badge_widths | all(. <= 18))
  and .first_contact_player_screen_label_guard.production_slot_status_labels == ["Q1 64 R","Q2 42 R","Q3 64 R","B2 42 R"]
  and .first_contact_player_screen_label_guard.production_empty_slot_status_labels == ["ADD UNIT","ADD UNIT","ADD BUILD","ADD BUILD"]
  and .first_contact_player_screen_label_guard.build_palette_labels == ["POWER","TRAIN","REFINE","TOWER","COMMAND","RADAR","WALL","SIGNAL"]
  and .first_contact_player_screen_label_guard.build_palette_badge_labels == ["PWR","TRN","REF","TWR","CMD","RAD","WAL","SIG"]
  and (.first_contact_player_screen_label_guard.build_palette_badge_widths | all(. <= 18))
  and .first_contact_player_screen_label_guard.build_palette_state_labels == ["READY","QUEUE","READY","QUEUE","READY","READY","READY","QUEUE"]
  and .first_contact_player_screen_label_guard.order_queue_labels == ["ATTACK BEACON","TRAIN WORKER","BUILD RELAY","MOVE 16/9"]
  and .first_contact_player_screen_label_guard.completion_event_labels == ["WORKER READY","SIGNAL READY","TOWER READY","TRAINING READY"]
  and .first_contact_player_screen_label_guard.tactics_queue_summary == "GUARD 64% TOWER 42%"
  and .first_contact_player_screen_label_guard.tactics_target_label == "RELAY BEACON"
  and .first_contact_player_screen_label_guard.tactics_build_label == "IDLE"
  and .first_contact_player_screen_label_guard.tactics_detail_labels == ["SECURE RELAY BEACON","RELAY BEACON","16/16","GUARD 64% TOWER 42%","IDLE"]
  and .first_contact_player_screen_label_guard.tactics_compact_badge_labels == ["SECURE","BEACON","16/16","G64/T42","IDLE"]
  and (.first_contact_player_screen_label_guard.tactics_compact_badge_widths | all(. <= 48))
  and .first_contact_player_screen_label_guard.tactics_queue_fallback_badge_labels == ["TRN SIG","TRN SIG","BLD RLY","ATK BCN","RDY"]
  and (.first_contact_player_screen_label_guard.tactics_queue_fallback_badge_widths | all(. <= 42))
  and .first_contact_player_screen_label_guard.field_status_title == "FIELD STATUS"
  and .first_contact_player_screen_label_guard.live_status_labels == ["SQUAD READY","RALLY 16/9","QUEUE GUARD 64%","SCOUTING 76%","CAMERA 16/16","SUPPLY 12/22","SAVE ROUTE READY"]
  and .first_contact_player_screen_label_guard.live_state_labels == ["ORDER SECURE RELAY BEACON","TARGET RELAY BEACON","BUILD IDLE","QUEUE GUARD 64%","CAM 16/16","DRAG NONE","HOVER NONE","RES UPGRADE 210 CREDITS"]
  and .first_contact_player_screen_label_guard.tactical_header_title == "TACTICAL VIEW"
  and .first_contact_player_screen_label_guard.tactical_header_order_label == "SECURE RELAY BEACON"
  and .first_contact_player_screen_label_guard.tactical_header_camera_label == "CAM 16/16 Z100"
  and (.first_contact_player_screen_label_guard.build_placement_status_label | startswith("PLACE "))
  and (.first_contact_player_screen_label_guard.build_placement_status_label | contains("TOWER"))
  and (.first_contact_player_screen_label_guard.build_placement_status_label | contains(":") | not)
  and (.first_contact_player_screen_label_guard.build_placement_status_label | contains("_") | not)
  and (.first_contact_player_screen_label_guard.upgrade_placement_status_label | contains("TRAINING"))
  and (.first_contact_player_screen_label_guard.upgrade_placement_status_label | contains(":") | not)
  and (.first_contact_player_screen_label_guard.upgrade_placement_status_label | contains("_") | not)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("TRNM") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("PRODUCTION COMPLETE") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("LIVE INPUT") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("LMB") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("WASD") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("@") != null)
  and (.first_contact_player_screen_label_guard.forbidden_display_fragments | index("_") != null)
  and (.first_contact_player_screen_label_guard.resource_spacing_samples | all(.value_spacing_gate == true))
  and (.first_contact_player_screen_label_guard.build_palette_fit_samples | all(.fits_tile_gate == true))
  and .first_contact_player_screen_label_guard.expected_label_gate == true
  and .first_contact_player_screen_label_guard.resource_spacing_gate == true
  and .first_contact_player_screen_label_guard.production_slot_width_gate == true
  and .first_contact_player_screen_label_guard.production_slot_status_width_gate == true
  and .first_contact_player_screen_label_guard.build_palette_width_gate == true
  and .first_contact_player_screen_label_guard.build_palette_state_width_gate == true
  and .first_contact_player_screen_label_guard.order_queue_width_gate == true
  and .first_contact_player_screen_label_guard.tactics_summary_width_gate == true
  and .first_contact_player_screen_label_guard.tactics_detail_width_gate == true
  and .first_contact_player_screen_label_guard.tactics_compact_badge_width_gate == true
  and .first_contact_player_screen_label_guard.tactics_queue_fallback_badge_width_gate == true
  and .first_contact_player_screen_label_guard.live_status_width_gate == true
  and .first_contact_player_screen_label_guard.live_state_width_gate == true
  and .first_contact_player_screen_label_guard.tactical_header_title_order_width_gate == true
  and .first_contact_player_screen_label_guard.tactical_header_camera_width_gate == true
  and .first_contact_player_screen_label_guard.build_placement_status_width_gate == true
  and .first_contact_player_screen_label_guard.tactical_header_vertical_separation_gate == true
  and .first_contact_player_screen_label_guard.raw_marker_gate == true
  and .first_contact_player_screen_label_guard_gate == true
  and .first_contact_visual_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_visual_readability_v1"
  and .first_contact_visual_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_visual_readability_v1"
  and .first_contact_visual_readability_guard.green == true
  and .first_contact_visual_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_readability_overlays"
  and .first_contact_visual_readability_guard.selected_tile_ids == ["14,11","15,11","15,12","17,12"]
  and .first_contact_visual_readability_guard.selected_marker_pixel_budget >= 224
  and .first_contact_visual_readability_guard.selected_marker_gate == true
  and .first_contact_visual_readability_guard.route_tile_ids == ["14,11","15,11","16,10","16,9"]
  and .first_contact_visual_readability_guard.route_marker_pixel_budget >= 72
  and .first_contact_visual_readability_guard.route_marker_gate == true
  and .first_contact_visual_readability_guard.command_destination_tile == "16,9"
  and .first_contact_visual_readability_guard.command_target_gate == true
  and (.first_contact_visual_readability_guard.structure_anchor_tiles | index("8,8") != null)
  and (.first_contact_visual_readability_guard.structure_anchor_tiles | index("25,25") != null)
  and (.first_contact_visual_readability_guard.structure_anchor_tiles | index("11,8") != null)
  and .first_contact_visual_readability_guard.structure_outline_pixel_budget >= 552
  and .first_contact_visual_readability_guard.structure_outline_gate == true
  and .first_contact_visual_readability_guard.objective_focus_tiles == ["16,9","16,24","9,16","24,16"]
  and .first_contact_visual_readability_guard.objective_focus_pixel_budget >= 288
  and .first_contact_visual_readability_guard.objective_focus_gate == true
  and (.first_contact_visual_readability_guard.lane_edge_sample_tiles | length) >= 12
  and .first_contact_visual_readability_guard.lane_edge_pixel_budget >= 192
  and .first_contact_visual_readability_guard.terrain_lane_edge_gate == true
  and .first_contact_visual_readability_guard_gate == true
  and .first_contact_radar_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_radar_readability_v1"
  and .first_contact_radar_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_radar_readability_v1"
  and .first_contact_radar_readability_guard.green == true
  and .first_contact_radar_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_radar_context"
  and .first_contact_radar_readability_guard.known_terrain_cell_count == 1024
  and .first_contact_radar_readability_guard.fog_context_cell_count >= 900
  and .first_contact_radar_readability_guard.visible_tile_gate == true
  and .first_contact_radar_readability_guard.selected_tile_ids == ["14,11","15,11","15,12","17,12"]
  and .first_contact_radar_readability_guard.selected_blip_gate == true
  and .first_contact_radar_readability_guard.route_tile_ids == ["14,11","15,11","16,10","16,9"]
  and .first_contact_radar_readability_guard.route_trace_gate == true
  and .first_contact_radar_readability_guard.command_destination_tile == "16,9"
  and .first_contact_radar_readability_guard.objective_tiles == ["16,9","16,24","9,16","24,16"]
  and .first_contact_radar_readability_guard.objective_blip_gate == true
  and (.first_contact_radar_readability_guard.structure_tiles | index("8,8") != null)
  and (.first_contact_radar_readability_guard.structure_tiles | index("25,25") != null)
  and .first_contact_radar_readability_guard.structure_blip_gate == true
  and .first_contact_radar_readability_guard.pressure_tiles == ["25,25","25,8","24,16"]
  and .first_contact_radar_readability_guard.pressure_blip_gate == true
  and (.first_contact_radar_readability_guard.lane_sample_tiles | length) >= 24
  and .first_contact_radar_readability_guard.lane_context_gate == true
  and .first_contact_radar_readability_guard.viewport_frame_gate == true
  and .first_contact_radar_readability_guard_gate == true
  and .first_contact_command_grid_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_command_grid_readability_v1"
  and .first_contact_command_grid_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_command_grid_readability_v1"
  and .first_contact_command_grid_readability_guard.green == true
  and .first_contact_command_grid_readability_guard.source_path == "trnm-world-bevy classic_draw_rts_command_glyph role-specific First Contact command buttons"
  and .first_contact_command_grid_readability_guard.command_slot_ids == ["worker","scout","warden","relay","core","signal","worker","scout","warden","relay","core","signal"]
  and .first_contact_command_grid_readability_guard.command_icon_roles == ["worker","scout","warden","relay","core","signal","worker","scout","warden","relay","core","signal"]
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("unit_pickaxe_ore") != null)
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("diamond_eye_crosshair") != null)
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("shield_barrier") != null)
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("mast_broadcast") != null)
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("stepped_base") != null)
  and (.first_contact_command_grid_readability_guard.command_icon_signatures | index("pulse_spire") != null)
  and .first_contact_command_grid_readability_guard.unique_icon_role_count >= 6
  and .first_contact_command_grid_readability_guard.unique_icon_signature_count >= 6
  and .first_contact_command_grid_readability_guard.top_row_roles == ["worker","scout","warden","relay","core","signal"]
  and .first_contact_command_grid_readability_guard.bottom_row_roles == ["worker","scout","warden","relay","core","signal"]
  and .first_contact_command_grid_readability_guard.active_slot_role == "worker"
  and (.first_contact_command_grid_readability_guard.cooldown_badge_samples | length) == 6
  and .first_contact_command_grid_readability_guard.slot_badge_pixel_budget >= 144
  and .first_contact_command_grid_readability_guard.glyph_shape_pixel_budget >= 1152
  and .first_contact_command_grid_readability_guard.role_sequence_gate == true
  and .first_contact_command_grid_readability_guard.repeated_rows_gate == true
  and .first_contact_command_grid_readability_guard.unique_icon_gate == true
  and .first_contact_command_grid_readability_guard.active_slot_gate == true
  and .first_contact_command_grid_readability_guard.cooldown_badge_gate == true
  and .first_contact_command_grid_readability_guard.player_screen_symbol_gate == true
  and .first_contact_command_grid_readability_guard_gate == true
  and .first_contact_sidebar_density_contract == "trillionnium_world_bevy_classic_rts_first_contact_sidebar_density_v1"
  and .first_contact_sidebar_density_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_sidebar_density_v1"
  and .first_contact_sidebar_density_guard.green == true
  and .first_contact_sidebar_density_guard.source_path == "trnm-world-bevy classic_draw_openra_style_rts_shell right sidebar production/build palette/tactics density"
  and .first_contact_sidebar_density_guard.production_slot_visible_count == 4
  and .first_contact_sidebar_density_guard.production_slot_column_count == 2
  and .first_contact_sidebar_density_guard.production_row_count == 2
  and .first_contact_sidebar_density_guard.production_slot_status_labels == ["Q1 64 R","Q2 42 R","Q3 64 R","B2 42 R"]
  and .first_contact_sidebar_density_guard.production_slot_status_badge_labels == ["Q1","Q2","Q3","B2"]
  and .first_contact_sidebar_density_guard.production_empty_slot_status_badge_labels == ["ADD","ADD","ADD","ADD"]
  and .first_contact_sidebar_density_guard.production_empty_slot_badge_label == "READY"
  and .first_contact_sidebar_density_guard.production_empty_slot_badge_width_px == 30
  and .first_contact_sidebar_density_guard.production_slot_labels == ["GUARD","WORKER","SIGNAL","TRAINING"]
  and .first_contact_sidebar_density_guard.production_slot_badge_labels == ["GRD","WRK","SIG","TRN"]
  and (.first_contact_sidebar_density_guard.production_slot_badge_widths | all(. <= 18))
  and (.first_contact_sidebar_density_guard.production_slot_status_badge_widths | all(. <= 18))
  and (.first_contact_sidebar_density_guard.production_empty_slot_status_badge_widths | all(. <= 18))
  and .first_contact_sidebar_density_guard.production_to_palette_gap_px >= 16
  and .first_contact_sidebar_density_guard.build_palette_visible_count == 8
  and .first_contact_sidebar_density_guard.build_palette_column_count == 4
  and .first_contact_sidebar_density_guard.build_palette_row_count == 2
  and .first_contact_sidebar_density_guard.build_palette_slot_width_px == 46
  and .first_contact_sidebar_density_guard.build_palette_slot_height_px == 40
  and .first_contact_sidebar_density_guard.build_palette_row_gap_px == 48
  and .first_contact_sidebar_density_guard.build_palette_inter_row_gap_px >= 8
  and .first_contact_sidebar_density_guard.build_palette_to_tactics_gap_px >= 24
  and .first_contact_sidebar_density_guard.build_palette_labels == ["POWER","TRAIN","REFINE","TOWER","COMMAND","RADAR","WALL","SIGNAL"]
  and .first_contact_sidebar_density_guard.build_palette_badge_labels == ["PWR","TRN","REF","TWR","CMD","RAD","WAL","SIG"]
  and (.first_contact_sidebar_density_guard.build_palette_badge_widths | all(. <= 18))
  and .first_contact_sidebar_density_guard.build_palette_state_labels == ["READY","QUEUE","READY","QUEUE","READY","READY","READY","QUEUE"]
  and .first_contact_sidebar_density_guard.build_palette_state_badge_labels == ["RDY","QUE","RDY","QUE","RDY","RDY","RDY","QUE"]
  and (.first_contact_sidebar_density_guard.build_palette_state_badge_widths | all(. <= 18))
  and .first_contact_sidebar_density_guard.build_palette_state_badge_width_px == 24
  and .first_contact_sidebar_density_guard.build_palette_state_badge_height_px == 9
  and .first_contact_sidebar_density_guard.tactics_row_count == 5
  and .first_contact_sidebar_density_guard.tactics_row_gap_px >= 4
  and .first_contact_sidebar_density_guard.tactics_detail_labels == ["SECURE RELAY BEACON","RELAY BEACON","16/16","GUARD 64% TOWER 42%","IDLE"]
  and .first_contact_sidebar_density_guard.tactics_compact_badge_labels == ["SECURE","BEACON","16/16","G64/T42","IDLE"]
  and (.first_contact_sidebar_density_guard.tactics_compact_badge_widths | all(. <= 48))
  and .first_contact_sidebar_density_guard.tactics_queue_fallback_badge_labels == ["TRN SIG","TRN SIG","BLD RLY","ATK BCN","RDY"]
  and (.first_contact_sidebar_density_guard.tactics_queue_fallback_badge_widths | all(. <= 48))
  and .first_contact_sidebar_density_guard.production_density_gate == true
  and .first_contact_sidebar_density_guard.palette_geometry_gate == true
  and .first_contact_sidebar_density_guard.palette_state_badge_gate == true
  and .first_contact_sidebar_density_guard.palette_label_gate == true
  and .first_contact_sidebar_density_guard.tactics_density_gate == true
  and .first_contact_sidebar_density_guard.right_sidebar_density_gate == true
  and .first_contact_sidebar_density_guard_gate == true
  and .first_contact_bottom_panel_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_bottom_panel_readability_v1"
  and .first_contact_bottom_panel_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_bottom_panel_readability_v1"
  and .first_contact_bottom_panel_readability_guard.green == true
  and .first_contact_bottom_panel_readability_guard.source_path == "trnm-world-bevy classic_draw_openra_style_rts_shell bottom selection/status panel"
  and .first_contact_bottom_panel_readability_guard.group_summary == "GROUP 1  4 UNITS SELECTED"
  and .first_contact_bottom_panel_readability_guard.selected_unit_display_count == 4
  and .first_contact_bottom_panel_readability_guard.upgrade_feedback_label == "SIGNAL BLADE READY"
  and .first_contact_bottom_panel_readability_guard.build_feedback_label == "WATCH TOWER READY"
  and .first_contact_bottom_panel_readability_guard.squad_role_labels == ["WORKER","SCOUT","GUARD","RELAY"]
  and .first_contact_bottom_panel_readability_guard.order_queue_badge_labels == ["ATK BCN","TRN WRK","BLD RLY","MOV 16/9"]
  and .first_contact_bottom_panel_readability_guard.completion_event_badge_labels == ["WRK READY","SIG READY","TWR READY","TRN READY"]
  and (.first_contact_bottom_panel_readability_guard.feedback_labels | index("SIGNAL BLADE READY") != null)
  and (.first_contact_bottom_panel_readability_guard.feedback_labels | index("WATCH TOWER READY") != null)
  and .first_contact_bottom_panel_readability_guard.raw_marker_gate == true
  and .first_contact_bottom_panel_readability_guard.feedback_expected_gate == true
  and .first_contact_bottom_panel_readability_guard.feedback_width_gate == true
  and .first_contact_bottom_panel_readability_guard.squad_strip_gate == true
  and .first_contact_bottom_panel_readability_guard.squad_chip_width_gate == true
  and .first_contact_bottom_panel_readability_guard.squad_chip_bottom_margin_px >= 16
  and .first_contact_bottom_panel_readability_guard.squad_chip_edge_clearance_gate == true
  and .first_contact_bottom_panel_readability_guard.order_queue_badge_gate == true
  and .first_contact_bottom_panel_readability_guard.selection_density_gate == true
  and .first_contact_bottom_panel_readability_guard_gate == true
  and .first_contact_silhouette_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_silhouette_readability_v1"
  and .first_contact_silhouette_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_silhouette_readability_v1"
  and .first_contact_silhouette_readability_guard.green == true
  and .first_contact_silhouette_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_silhouette_readability_layer"
  and .first_contact_silhouette_readability_guard.unit_roles == ["worker","scout","warden","relay"]
  and .first_contact_silhouette_readability_guard.unit_signatures == ["cargo_pack","sensor_mast","shield_plate","relay_courier"]
  and .first_contact_silhouette_readability_guard.command_core_silhouette_count == 4
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_samples == [{"tile":"8,8","role":"command_core","signature":"stepped_roof_core"},{"tile":"25,8","role":"command_core","signature":"stepped_roof_core"},{"tile":"25,25","role":"command_core","signature":"stepped_roof_core"},{"tile":"8,25","role":"command_core","signature":"stepped_roof_core"}]
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_count == 4
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_ticks_per_core == 4
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_tick_width_px == 6
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_tick_height_px == 2
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_pixel_budget <= 192
  and .first_contact_silhouette_readability_guard.player_screen_command_core_hot_roof_pixel_budget == 0
  and (.first_contact_silhouette_readability_guard.player_screen_command_core_faction_signatures | index("player_screen_command_core_faction_micro_ticks") != null)
  and (.first_contact_silhouette_readability_guard.player_screen_command_core_faction_signatures | index("player_screen_command_core_roof_body_muted") != null)
  and .first_contact_silhouette_readability_guard.relay_silhouette_count == 2
  and .first_contact_silhouette_readability_guard.beacon_silhouette_count == 4
  and .first_contact_silhouette_readability_guard.player_screen_target_beacon_tile == "16,9"
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_samples == [{"tile":"16,24","role":"beacon","signature":"vertical_beacon_spire"},{"tile":"9,16","role":"beacon","signature":"vertical_beacon_spire"},{"tile":"24,16","role":"beacon","signature":"vertical_beacon_spire"}]
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_count == 3
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_cues_per_beacon == 5
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_cue_width_px == 8
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_cue_height_px == 2
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_pixel_budget <= 240
  and (.first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_signatures | index("player_screen_secondary_beacon_body_micro_cues") != null)
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_samples == [{"tile":"16,24","role":"beacon_ring","signature":"beacon_ring_actor_body"},{"tile":"9,16","role":"beacon_ring","signature":"beacon_ring_actor_body"},{"tile":"24,16","role":"beacon_ring","signature":"beacon_ring_actor_body"}]
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_count == 3
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_cues_per_beacon == 4
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_cue_width_px == 8
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_cue_height_px == 2
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_pixel_budget <= 192
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_hot_actor_body_pixel_budget == 0
  and (.first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_signatures | index("player_screen_secondary_beacon_actor_body_micro_cues") != null)
  and (.first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_signatures | index("player_screen_secondary_beacon_hot_actor_body_suppressed") != null)
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_count == 11
  and (.first_contact_silhouette_readability_guard.preview_resource_signatures | index("resource_bloom_micro_shadow_cues") != null)
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_cues_per_cluster == 4
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_cue_width_px == 6
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_cue_height_px == 2
  and (.first_contact_silhouette_readability_guard.terrain_signatures | index("base_corner_frame") != null)
  and (.first_contact_silhouette_readability_guard.terrain_signatures | index("flux_glint_cluster") != null)
  and (.first_contact_silhouette_readability_guard.terrain_signatures | index("basin_cross_rim") != null)
  and (.first_contact_silhouette_readability_guard.structure_signatures | index("stepped_roof_core") != null)
  and (.first_contact_silhouette_readability_guard.structure_signatures | index("tall_signal_mast") != null)
  and (.first_contact_silhouette_readability_guard.structure_signatures | index("vertical_beacon_spire") != null)
  and .first_contact_silhouette_readability_guard.terrain_zone_pixel_budget >= 288
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_pixel_budget >= 528
  and .first_contact_silhouette_readability_guard.unit_silhouette_pixel_budget >= 344
  and .first_contact_silhouette_readability_guard.structure_roofline_pixel_budget >= 960
  and .first_contact_silhouette_readability_guard.beacon_spire_pixel_budget >= 288
  and .first_contact_silhouette_readability_guard.terrain_zone_gate == true
  and .first_contact_silhouette_readability_guard.preview_resource_bloom_gate == true
  and .first_contact_silhouette_readability_guard.unit_role_silhouette_gate == true
  and .first_contact_silhouette_readability_guard.structure_roofline_gate == true
  and .first_contact_silhouette_readability_guard.player_screen_command_core_faction_gate == true
  and .first_contact_silhouette_readability_guard.beacon_spire_gate == true
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_body_gate == true
  and .first_contact_silhouette_readability_guard.player_screen_secondary_beacon_actor_body_gate == true
  and .first_contact_silhouette_readability_guard.map_object_silhouette_gate == true
  and .first_contact_silhouette_readability_guard_gate == true
  and .first_contact_art_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_art_readability_v1"
  and .first_contact_art_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_art_readability_v1"
  and .first_contact_art_readability_guard.green == true
  and .first_contact_art_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_art_readability_layer"
  and .first_contact_art_readability_guard.terrain_material_roles == ["base_concrete","base_concrete","base_concrete","base_concrete","resource_crystal","resource_crystal","beacon_lane","beacon_lane","basin_floor"]
  and .first_contact_art_readability_guard.building_roles == ["command_core","command_core","command_core","command_core","relay","relay","beacon","beacon","beacon","beacon"]
  and .first_contact_art_readability_guard.map_landmark_roles == ["base_gate","base_gate","base_gate","base_gate","resource_cluster","resource_cluster","beacon_lane","beacon_lane","basin_scar","basin_scar","relay_cable","relay_cable","beacon_ring","beacon_ring","beacon_ring","beacon_ring"]
  and (.first_contact_art_readability_guard.terrain_material_signatures | index("foundation_panel_seams") != null)
  and (.first_contact_art_readability_guard.terrain_material_signatures | index("flux_crystal_shards") != null)
  and (.first_contact_art_readability_guard.terrain_material_signatures | index("painted_lane_chevrons") != null)
  and (.first_contact_art_readability_guard.terrain_material_signatures | index("cracked_plaza_cross") != null)
  and .first_contact_art_readability_guard.terrain_material_depth_source_path == "trnm-world-bevy classic_draw_first_contact_terrain_material_depth_detail"
  and (.first_contact_art_readability_guard.terrain_material_depth_signatures | index("terrain_foundation_beveled_edges") != null)
  and (.first_contact_art_readability_guard.terrain_material_depth_signatures | index("terrain_crystal_cast_shadows") != null)
  and (.first_contact_art_readability_guard.terrain_material_depth_signatures | index("terrain_lane_recessed_rails") != null)
  and (.first_contact_art_readability_guard.terrain_material_depth_signatures | index("terrain_basin_fracture_shadows") != null)
  and (.first_contact_art_readability_guard.building_facade_signatures | index("lit_window_rows") != null)
  and (.first_contact_art_readability_guard.building_facade_signatures | index("antenna_band_panels") != null)
  and (.first_contact_art_readability_guard.building_facade_signatures | index("glowing_spire_panels") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("base_gate_lamps") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("crystal_shadow_sparkles") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("lane_power_pylons") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("crater_scuff_marks") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("relay_ground_cables") != null)
  and (.first_contact_art_readability_guard.map_landmark_signatures | index("beacon_capture_rings") != null)
  and .first_contact_art_readability_guard.command_core_facade_count == 4
  and .first_contact_art_readability_guard.relay_facade_count == 2
  and .first_contact_art_readability_guard.beacon_facade_count == 4
  and .first_contact_art_readability_guard.base_landmark_count == 4
  and .first_contact_art_readability_guard.resource_landmark_count == 2
  and .first_contact_art_readability_guard.lane_landmark_count == 2
  and .first_contact_art_readability_guard.basin_landmark_count == 2
  and .first_contact_art_readability_guard.relay_landmark_count == 2
  and .first_contact_art_readability_guard.beacon_landmark_count == 4
  and .first_contact_art_readability_guard.terrain_material_pixel_budget >= 432
  and .first_contact_art_readability_guard.terrain_material_depth_sample_count >= 9
  and .first_contact_art_readability_guard.terrain_material_depth_pixel_budget >= 576
  and .first_contact_art_readability_guard.building_facade_pixel_budget >= 860
  and .first_contact_art_readability_guard.map_landmark_pixel_budget >= 1152
  and .first_contact_art_readability_guard.runtime_actor_depth_source_path == "trnm-world-bevy classic_draw_first_contact_actor_glyph"
  and (.first_contact_art_readability_guard.runtime_actor_depth_signatures | index("runtime_structure_roof_rim") != null)
  and (.first_contact_art_readability_guard.runtime_actor_depth_signatures | index("runtime_structure_side_shadow") != null)
  and (.first_contact_art_readability_guard.runtime_actor_depth_signatures | index("runtime_command_window_pips") != null)
  and (.first_contact_art_readability_guard.runtime_actor_depth_signatures | index("runtime_relay_mast_braces") != null)
  and (.first_contact_art_readability_guard.runtime_actor_depth_signatures | index("runtime_beacon_core_glow_rungs") != null)
  and .first_contact_art_readability_guard.runtime_actor_depth_pixel_budget >= 480
  and .first_contact_art_readability_guard.lower_secondary_beacon_art_samples == [{"tile":"16,24","role":"beacon_lane","signature":"painted_lane_chevrons"},{"tile":"16,23","role":"beacon_lane","signature":"lane_power_pylons"}]
  and .first_contact_art_readability_guard.lower_secondary_beacon_art_sample_count == 2
  and .first_contact_art_readability_guard.lower_secondary_beacon_art_pixel_budget <= 64
  and (.first_contact_art_readability_guard.lower_secondary_beacon_art_signatures | index("lower_secondary_beacon_micro_chevrons") != null)
  and (.first_contact_art_readability_guard.lower_secondary_beacon_art_signatures | index("lower_secondary_beacon_power_pylons_capped") != null)
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_samples == [{"tile":"16,24","role":"beacon_ring","signature":"beacon_capture_rings"},{"tile":"9,16","role":"beacon_ring","signature":"beacon_capture_rings"},{"tile":"24,16","role":"beacon_ring","signature":"beacon_capture_rings"}]
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_count == 3
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_cues_per_ring == 4
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_cue_count == 12
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_cue_width_px == 8
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_cue_height_px == 2
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_pixel_budget <= 192
  and (.first_contact_art_readability_guard.secondary_beacon_capture_ring_signatures | index("secondary_beacon_capture_micro_cues") != null)
  and .first_contact_art_readability_guard.player_screen_command_core_art_samples == [{"tile":"8,8","role":"command_core","signature":"lit_window_rows"},{"tile":"25,8","role":"command_core","signature":"lit_window_rows"},{"tile":"25,25","role":"command_core","signature":"lit_window_rows"},{"tile":"8,25","role":"command_core","signature":"lit_window_rows"}]
  and .first_contact_art_readability_guard.player_screen_command_core_art_count == 4
  and .first_contact_art_readability_guard.player_screen_command_core_art_ticks_per_core == 4
  and .first_contact_art_readability_guard.player_screen_command_core_art_tick_width_px == 6
  and .first_contact_art_readability_guard.player_screen_command_core_art_tick_height_px == 2
  and .first_contact_art_readability_guard.player_screen_command_core_art_pixel_budget <= 192
  and .first_contact_art_readability_guard.player_screen_command_core_hot_facade_pixel_budget == 0
  and (.first_contact_art_readability_guard.player_screen_command_core_art_signatures | index("player_screen_command_core_art_micro_ticks") != null)
  and (.first_contact_art_readability_guard.player_screen_command_core_art_signatures | index("player_screen_command_core_hot_facade_suppressed") != null)
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_samples == [{"tile":"8,9","role":"base_gate","signature":"base_gate_lamps"},{"tile":"25,9","role":"base_gate","signature":"base_gate_lamps"},{"tile":"25,24","role":"base_gate","signature":"base_gate_lamps"},{"tile":"8,24","role":"base_gate","signature":"base_gate_lamps"}]
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_count == 4
  and .first_contact_art_readability_guard.player_screen_base_gate_lamps_per_gate == 2
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_width_px == 6
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_height_px == 2
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_pixel_budget <= 96
  and .first_contact_art_readability_guard.player_screen_base_gate_hot_lamp_pixel_budget == 0
  and (.first_contact_art_readability_guard.player_screen_base_gate_lamp_signatures | index("player_screen_base_gate_micro_lamps") != null)
  and (.first_contact_art_readability_guard.player_screen_base_gate_lamp_signatures | index("player_screen_base_gate_hot_lamps_suppressed") != null)
  and .first_contact_art_readability_guard.terrain_material_gate == true
  and .first_contact_art_readability_guard.terrain_material_depth_gate == true
  and .first_contact_art_readability_guard.building_facade_gate == true
  and .first_contact_art_readability_guard.map_landmark_detail_gate == true
  and .first_contact_art_readability_guard.runtime_actor_depth_gate == true
  and .first_contact_art_readability_guard.lower_secondary_beacon_art_deemphasis_gate == true
  and .first_contact_art_readability_guard.secondary_beacon_capture_ring_gate == true
  and .first_contact_art_readability_guard.player_screen_command_core_art_gate == true
  and .first_contact_art_readability_guard.player_screen_base_gate_lamp_gate == true
  and .first_contact_art_readability_guard.authored_map_art_gate == true
  and .first_contact_art_readability_guard_gate == true
  and .first_contact_motion_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_motion_readability_v1"
  and .first_contact_motion_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_motion_readability_v1"
  and .first_contact_motion_readability_guard.green == true
  and .first_contact_motion_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_opening_actions + classic_draw_first_contact_unit_state_layers + classic_draw_first_contact_combat_phase_layers + classic_draw_first_contact_command_feedback_layers + classic_draw_first_contact_animation_readability_layer"
  and .first_contact_motion_readability_guard.opening_action_ids == ["worker_harvest_flux","build_flux_relay","train_worker","train_horizon_scout","secure_flux_beacon"]
  and .first_contact_motion_readability_guard.action_verbs == ["worker","build","train","train","secure"]
  and .first_contact_motion_readability_guard.unit_status_badges == ["W","S","R","G"]
  and .first_contact_motion_readability_guard.unit_status_color_roles == ["health","mana","attack","confirm"]
  and (.first_contact_motion_readability_guard.unit_state_motion_signatures | index("player_screen_production_training_micro_sparks") != null)
  and .first_contact_motion_readability_guard.production_training_spark_count == 3
  and .first_contact_motion_readability_guard.production_training_spark_width_px == 4
  and .first_contact_motion_readability_guard.production_training_spark_height_px == 2
  and .first_contact_motion_readability_guard.production_training_spark_pixel_budget <= 24
  and (.first_contact_motion_readability_guard.unit_state_motion_signatures | index("player_screen_warden_attack_micro_sparks") != null)
  and .first_contact_motion_readability_guard.warden_attack_arm_count == 3
  and .first_contact_motion_readability_guard.warden_attack_arm_width_px == 14
  and .first_contact_motion_readability_guard.warden_attack_arm_height_px == 2
  and .first_contact_motion_readability_guard.warden_attack_arm_pixel_budget <= 84
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_animation_training_lane_micro_ticks") != null)
  and .first_contact_motion_readability_guard.animation_training_tick_count == 3
  and .first_contact_motion_readability_guard.animation_training_tick_width_px == 4
  and .first_contact_motion_readability_guard.animation_training_tick_height_px == 2
  and .first_contact_motion_readability_guard.animation_training_tick_pixel_budget <= 24
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_spawn_door_micro_shutters") != null)
  and .first_contact_motion_readability_guard.spawn_door_shutter_count == 7
  and .first_contact_motion_readability_guard.spawn_door_shutter_width_px == 4
  and .first_contact_motion_readability_guard.spawn_door_shutter_height_px == 2
  and .first_contact_motion_readability_guard.spawn_door_shutter_pixel_budget <= 56
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_rally_flag_micro_pennants") != null)
  and .first_contact_motion_readability_guard.rally_flag_pennant_count == 5
  and .first_contact_motion_readability_guard.rally_flag_pennant_width_px == 6
  and .first_contact_motion_readability_guard.rally_flag_pennant_height_px == 2
  and .first_contact_motion_readability_guard.rally_flag_pennant_pixel_budget <= 60
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_formation_join_micro_pips") != null)
  and .first_contact_motion_readability_guard.formation_join_pip_count == 3
  and .first_contact_motion_readability_guard.formation_join_pip_width_px == 4
  and .first_contact_motion_readability_guard.formation_join_pip_height_px == 2
  and .first_contact_motion_readability_guard.formation_join_pip_pixel_budget <= 24
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_shield_charge_micro_arcs") != null)
  and .first_contact_motion_readability_guard.shield_charge_arc_count == 3
  and .first_contact_motion_readability_guard.shield_charge_arc_width_px == 10
  and .first_contact_motion_readability_guard.shield_charge_arc_height_px == 2
  and .first_contact_motion_readability_guard.shield_charge_arc_pixel_budget <= 60
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_sensor_sweep_micro_ticks") != null)
  and .first_contact_motion_readability_guard.sensor_sweep_tick_count == 3
  and .first_contact_motion_readability_guard.sensor_sweep_tick_width_px == 8
  and .first_contact_motion_readability_guard.sensor_sweep_tick_height_px == 2
  and .first_contact_motion_readability_guard.sensor_sweep_tick_pixel_budget <= 48
  and (.first_contact_motion_readability_guard.combat_phase_motion_signatures | index("player_screen_combat_hit_micro_sparks") != null)
  and .first_contact_motion_readability_guard.combat_hit_flash_site_count == 4
  and .first_contact_motion_readability_guard.combat_hit_flash_sparks_per_site == 2
  and .first_contact_motion_readability_guard.combat_hit_flash_spark_count == 8
  and .first_contact_motion_readability_guard.combat_hit_flash_spark_width_px == 6
  and .first_contact_motion_readability_guard.combat_hit_flash_spark_height_px == 2
  and .first_contact_motion_readability_guard.combat_hit_flash_spark_pixel_budget <= 96
  and .first_contact_motion_readability_guard.track_roles == ["action_trail","npc_action","action_trail","npc_action","action_trail","npc_action"]
  and .first_contact_motion_readability_guard.animation_roles == ["worker","worker","worker","scout","scout","warden","warden","relay","command_core","command_core","relay_structure","beacon","beacon"]
  and (.first_contact_motion_readability_guard.animation_signatures | index("harvest_tool_swing_frame") != null)
  and (.first_contact_motion_readability_guard.animation_signatures | index("sensor_sweep_arc") != null)
  and (.first_contact_motion_readability_guard.animation_signatures | index("spawn_door_open_frame") != null)
  and (.first_contact_motion_readability_guard.animation_signatures | index("capture_pulse_frame") != null)
  and .first_contact_motion_readability_guard.animation_frame_richness_source_path == "trnm-world-bevy classic_draw_first_contact_animation_frame_richness_detail"
  and (.first_contact_motion_readability_guard.animation_frame_richness_signatures | index("animation_secondary_pose_offsets") != null)
  and (.first_contact_motion_readability_guard.animation_frame_richness_signatures | index("animation_contact_smear_ticks") != null)
  and (.first_contact_motion_readability_guard.animation_frame_richness_signatures | index("animation_structure_shutter_frames") != null)
  and (.first_contact_motion_readability_guard.animation_frame_richness_signatures | index("animation_objective_afterglow_frames") != null)
  and .first_contact_motion_readability_guard.action_trail_count == 3
  and .first_contact_motion_readability_guard.npc_action_count == 3
  and .first_contact_motion_readability_guard.unit_animation_frame_count == 8
  and .first_contact_motion_readability_guard.building_animation_frame_count == 3
  and .first_contact_motion_readability_guard.objective_animation_frame_count == 2
  and .first_contact_motion_readability_guard.unique_animation_signature_count >= 13
  and .first_contact_motion_readability_guard.feedback_selected_group == "GROUP 1"
  and .first_contact_motion_readability_guard.feedback_active_order == "SECURE BEACON"
  and .first_contact_motion_readability_guard.feedback_player_labels == ["GROUP 1 SECURING BEACON","QUEUE ADDED  ORDER READY","ROUTE BLOCKED MID VENT"]
  and .first_contact_motion_readability_guard.feedback_target_tile == "16,9"
  and .first_contact_motion_readability_guard.feedback_queued_after > .first_contact_motion_readability_guard.feedback_queued_before
  and .first_contact_motion_readability_guard.feedback_command_ack_progress > .first_contact_motion_readability_guard.feedback_cooldown_progress
  and .first_contact_motion_readability_guard.route_tile_ids == ["14,11","15,11","16,10","16,9"]
  and .first_contact_motion_readability_guard.command_destination_tile == "16,9"
  and .first_contact_motion_readability_guard.attack_target_id == "trnm.flux.beacon"
  and (.first_contact_motion_readability_guard.combat_event_log | index("worker_carry_supply") != null)
  and (.first_contact_motion_readability_guard.combat_event_log | index("secure_beacon:16,9") != null)
  and .first_contact_motion_readability_guard.progress_meter_pixel_budget >= 200
  and (.first_contact_motion_readability_guard.opening_action_motion_signatures | index("opening_action_path_micro_dots") != null)
  and .first_contact_motion_readability_guard.opening_action_path_count == 3
  and .first_contact_motion_readability_guard.opening_action_path_step_count == 24
  and .first_contact_motion_readability_guard.opening_action_path_dot_width_px == 2
  and .first_contact_motion_readability_guard.opening_action_path_dot_height_px == 2
  and .first_contact_motion_readability_guard.opening_action_path_pixel_budget <= 96
  and .first_contact_motion_readability_guard.opening_action_path_gate == true
  and .first_contact_motion_readability_guard.unit_status_pixel_budget >= 256
  and .first_contact_motion_readability_guard.primary_tactical_track_count == 1
  and .first_contact_motion_readability_guard.secondary_tactical_track_count == 5
  and .first_contact_motion_readability_guard.primary_tactical_track_pixel_budget >= 48
  and .first_contact_motion_readability_guard.secondary_tactical_track_pixel_budget <= 80
  and .first_contact_motion_readability_guard.tactical_track_pixel_budget <= 128
  and .first_contact_motion_readability_guard.secondary_tactical_track_height_px == 1
  and .first_contact_motion_readability_guard.secondary_tactical_track_darken_numerator == 3
  and .first_contact_motion_readability_guard.secondary_tactical_track_darken_denominator == 4
  and (.first_contact_motion_readability_guard.tactical_track_density_signatures | index("primary_relay_beacon_track_kept_hot") != null)
  and (.first_contact_motion_readability_guard.tactical_track_density_signatures | index("secondary_tracks_dimmed") != null)
  and (.first_contact_motion_readability_guard.tactical_track_density_signatures | index("secondary_tracks_faded_into_terrain") != null)
  and (.first_contact_motion_readability_guard.tactical_track_density_signatures | index("secondary_tracks_one_pixel") != null)
  and .first_contact_motion_readability_guard.feedback_pixel_budget >= 190
  and (.first_contact_motion_readability_guard.command_feedback_motion_signatures | index("battlefield_command_feedback_micro_trail") != null)
  and (.first_contact_motion_readability_guard.command_feedback_motion_signatures | index("command_feedback_panel_micro_cues") != null)
  and .first_contact_motion_readability_guard.feedback_panel_cue_count == 8
  and .first_contact_motion_readability_guard.feedback_panel_cue_width_px == 6
  and .first_contact_motion_readability_guard.feedback_panel_cue_height_px == 2
  and .first_contact_motion_readability_guard.feedback_panel_cue_pixel_budget <= 96
  and (.first_contact_motion_readability_guard.command_feedback_motion_signatures | index("command_feedback_panel_backplate_suppressed") != null)
  and .first_contact_motion_readability_guard.feedback_panel_backplate_width_px == 0
  and .first_contact_motion_readability_guard.feedback_panel_backplate_height_px == 0
  and .first_contact_motion_readability_guard.feedback_panel_backplate_pixel_budget == 0
  and (.first_contact_motion_readability_guard.command_feedback_motion_signatures | index("command_feedback_progress_micro_pips") != null)
  and .first_contact_motion_readability_guard.feedback_progress_pip_slot_count == 7
  and .first_contact_motion_readability_guard.feedback_ack_progress_pip_count == 6
  and .first_contact_motion_readability_guard.feedback_cooldown_pip_count == 2
  and .first_contact_motion_readability_guard.feedback_progress_pip_count == 8
  and .first_contact_motion_readability_guard.feedback_progress_pip_width_px == 6
  and .first_contact_motion_readability_guard.feedback_progress_pip_height_px == 2
  and .first_contact_motion_readability_guard.feedback_progress_pip_pixel_budget <= 96
  and (.first_contact_motion_readability_guard.command_feedback_motion_signatures | index("command_feedback_progress_backplates_suppressed") != null)
  and .first_contact_motion_readability_guard.feedback_progress_backplate_width_px == 0
  and .first_contact_motion_readability_guard.feedback_progress_backplate_height_px == 0
  and .first_contact_motion_readability_guard.feedback_progress_backplate_pixel_budget == 0
  and .first_contact_motion_readability_guard.feedback_move_trail_origin_count == 2
  and .first_contact_motion_readability_guard.feedback_move_trail_step_count_per_origin == 10
  and .first_contact_motion_readability_guard.feedback_move_trail_tick_count == 20
  and .first_contact_motion_readability_guard.feedback_move_trail_tick_width_px == 2
  and .first_contact_motion_readability_guard.feedback_move_trail_tick_height_px == 2
  and .first_contact_motion_readability_guard.feedback_move_trail_pixel_budget <= 80
  and .first_contact_motion_readability_guard.feedback_battlefield_trail_gate == true
  and .first_contact_motion_readability_guard.animation_frame_pixel_budget >= 1144
  and .first_contact_motion_readability_guard.animation_frame_richness_sample_count >= 13
  and .first_contact_motion_readability_guard.animation_frame_richness_pixel_budget >= 520
  and .first_contact_motion_readability_guard.opening_action_gate == true
  and .first_contact_motion_readability_guard.unit_state_motion_gate == true
  and .first_contact_motion_readability_guard.production_training_spark_gate == true
  and .first_contact_motion_readability_guard.warden_attack_arm_gate == true
  and .first_contact_motion_readability_guard.animation_training_tick_gate == true
  and .first_contact_motion_readability_guard.spawn_door_shutter_gate == true
  and .first_contact_motion_readability_guard.rally_flag_pennant_gate == true
  and .first_contact_motion_readability_guard.formation_join_pip_gate == true
  and (.first_contact_motion_readability_guard.player_screen_animation_signatures | index("player_screen_worker_carry_load_micro_pips") != null)
  and .first_contact_motion_readability_guard.carry_load_pip_count == 4
  and .first_contact_motion_readability_guard.carry_load_pip_width_px == 4
  and .first_contact_motion_readability_guard.carry_load_pip_height_px == 2
  and .first_contact_motion_readability_guard.carry_load_pip_pixel_budget <= 32
  and .first_contact_motion_readability_guard.carry_load_pip_gate == true
  and .first_contact_motion_readability_guard.shield_charge_arc_gate == true
  and .first_contact_motion_readability_guard.sensor_sweep_tick_gate == true
  and .first_contact_motion_readability_guard.combat_hit_flash_spark_gate == true
  and .first_contact_motion_readability_guard.tactical_track_motion_gate == true
  and .first_contact_motion_readability_guard.command_feedback_motion_gate == true
  and .first_contact_motion_readability_guard.feedback_raw_marker_gate == true
  and .first_contact_motion_readability_guard.feedback_player_label_gate == true
  and .first_contact_motion_readability_guard.feedback_panel_micro_cue_gate == true
  and .first_contact_motion_readability_guard.feedback_panel_backplate_gate == true
  and .first_contact_motion_readability_guard.feedback_progress_micro_pip_gate == true
  and .first_contact_motion_readability_guard.feedback_progress_backplate_gate == true
  and .first_contact_motion_readability_guard.runtime_motion_gate == true
  and .first_contact_motion_readability_guard.unit_animation_frame_gate == true
  and .first_contact_motion_readability_guard.building_animation_frame_gate == true
  and .first_contact_motion_readability_guard.objective_animation_frame_gate == true
  and .first_contact_motion_readability_guard.animation_cycle_detail_gate == true
  and .first_contact_motion_readability_guard.animation_frame_richness_gate == true
  and .first_contact_motion_readability_guard_gate == true
  and .first_contact_atlas_readability_contract == "trillionnium_world_bevy_classic_rts_first_contact_atlas_readability_v1"
  and .first_contact_atlas_readability_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_atlas_readability_v1"
  and .first_contact_atlas_readability_guard.green == true
  and .first_contact_atlas_readability_guard.source_path == "trnm-world-bevy classic_draw_first_contact_atlas_readability_layer"
  and .first_contact_atlas_readability_guard.asset_pack_contract == "trillionnium_world_bevy_classic_asset_pack_v1"
  and (.first_contact_atlas_readability_guard.asset_boundary | contains("project_owned"))
  and .first_contact_atlas_readability_guard.atlas_parse_gate == true
  and .first_contact_atlas_readability_guard.atlas_roles == ["terrain_tile","terrain_tile","terrain_tile","terrain_tile","terrain_tile","terrain_tile","unit_sprite","unit_sprite","unit_sprite","unit_sprite","structure_sprite","structure_sprite","structure_sprite","structure_sprite","objective_sprite","objective_sprite"]
  and .first_contact_atlas_readability_guard.atlas_family_roles == ["worker_unit_family","worker_unit_family","scout_unit_family","scout_unit_family","warden_unit_family","warden_unit_family","relay_unit_family","command_core_structure_family","command_core_structure_family","relay_structure_family","relay_structure_family","beacon_objective_family","beacon_objective_family","beacon_objective_family"]
  and .first_contact_atlas_readability_guard.atlas_family_sample_tiles == ["4,14","4,16","4,18","4,20","29,14","29,16","29,18","4,4","6,4","27,4","29,4","29,22","29,24","29,26"]
  and .first_contact_atlas_readability_guard.atlas_family_gallery_lanes == ["west_gallery","west_gallery","west_gallery","west_gallery","east_gallery","east_gallery","east_gallery","north_gallery","north_gallery","north_gallery","north_gallery","east_gallery","east_gallery","east_gallery"]
  and .first_contact_atlas_readability_guard.atlas_family_busy_core_tiles == []
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("tile_stone") != null)
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("tile_water") != null)
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("actor_player_walk_south_1") != null)
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("actor_enemy_attack") != null)
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("prop_workbench") != null)
  and (.first_contact_atlas_readability_guard.atlas_frame_ids | index("marker_objective") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_frame_ids | index("actor_worker_carry") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_frame_ids | index("actor_guard_attack") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_frame_ids | index("model_town_hall") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_frame_ids | index("model_waygate") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_frame_ids | index("rts_command_destination_marker") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_override_frame_ids | index("actor_worker_carry") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_override_frame_ids | index("actor_guard_attack") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_override_frame_ids | index("model_town_hall") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_override_frame_ids | index("model_waygate") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_override_frame_ids | index("rts_command_destination_marker") != null)
  and (.first_contact_atlas_readability_guard.atlas_signatures | index("worker_walk_atlas_frame") != null)
  and (.first_contact_atlas_readability_guard.atlas_signatures | index("command_core_workbench_frame") != null)
  and (.first_contact_atlas_readability_guard.atlas_signatures | index("beacon_objective_atlas_frame") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_signatures | index("worker_carry_frame_family") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_signatures | index("warden_guard_attack_frame_family") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_signatures | index("command_core_town_hall_frame_family") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_signatures | index("relay_waygate_frame_family") != null)
  and (.first_contact_atlas_readability_guard.atlas_family_signatures | index("beacon_destination_marker_frame_family") != null)
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_samples == [{"tile":"16,24","role":"objective_sprite","frame_id":"marker_interaction","signature":"beacon_interaction_atlas_frame","scale":2}]
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_sample_count == 1
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_source_frame_pixel_budget >= 1024
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_rendered_frame_pixel_budget == 0
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_anchor_pixel_budget <= 24
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_depth_suppressed_count == 1
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_depth_pixel_budget == 0
  and (.first_contact_atlas_readability_guard.secondary_objective_atlas_signatures | index("secondary_objective_atlas_frame_suppressed") != null)
  and (.first_contact_atlas_readability_guard.secondary_objective_atlas_signatures | index("secondary_objective_atlas_micro_anchor_only") != null)
  and (.first_contact_atlas_readability_guard.secondary_objective_atlas_signatures | index("secondary_objective_atlas_depth_suppressed") != null)
  and .first_contact_atlas_readability_guard.terrain_frame_count == 6
  and .first_contact_atlas_readability_guard.unit_frame_count == 4
  and .first_contact_atlas_readability_guard.structure_frame_count == 4
  and .first_contact_atlas_readability_guard.objective_frame_count == 2
  and .first_contact_atlas_readability_guard.worker_family_frame_count == 2
  and .first_contact_atlas_readability_guard.scout_family_frame_count == 2
  and .first_contact_atlas_readability_guard.warden_family_frame_count == 2
  and .first_contact_atlas_readability_guard.relay_unit_family_frame_count == 1
  and .first_contact_atlas_readability_guard.command_core_family_frame_count == 2
  and .first_contact_atlas_readability_guard.relay_structure_family_frame_count == 2
  and .first_contact_atlas_readability_guard.beacon_family_frame_count == 3
  and .first_contact_atlas_readability_guard.unique_frame_count >= 14
  and .first_contact_atlas_readability_guard.unique_signature_count >= 12
  and .first_contact_atlas_readability_guard.atlas_family_unique_frame_count >= 14
  and .first_contact_atlas_readability_guard.atlas_family_unique_signature_count >= 14
  and .first_contact_atlas_readability_guard.atlas_family_unique_gallery_lane_count == 3
  and .first_contact_atlas_readability_guard.north_gallery_frame_count == 4
  and .first_contact_atlas_readability_guard.west_gallery_frame_count == 4
  and .first_contact_atlas_readability_guard.east_gallery_frame_count == 6
  and .first_contact_atlas_readability_guard.atlas_frame_pixel_budget >= 11776
  and .first_contact_atlas_readability_guard.atlas_family_frame_pixel_budget >= 42496
  and .first_contact_atlas_readability_guard.atlas_runtime_depth_source_path == "trnm-world-bevy classic_draw_first_contact_atlas_asset_sample"
  and (.first_contact_atlas_readability_guard.atlas_runtime_depth_signatures | index("atlas_unit_grounding_shadow") != null)
  and (.first_contact_atlas_readability_guard.atlas_runtime_depth_signatures | index("atlas_structure_footprint_rim") != null)
  and (.first_contact_atlas_readability_guard.atlas_runtime_depth_signatures | index("atlas_objective_capture_underlay") != null)
  and (.first_contact_atlas_readability_guard.atlas_runtime_depth_signatures | index("atlas_lower_lane_depth_suppressed") != null)
  and .first_contact_atlas_readability_guard.atlas_runtime_depth_sample_count >= 20
  and .first_contact_atlas_readability_guard.atlas_lower_lane_depth_suppressed_count == 3
  and .first_contact_atlas_readability_guard.atlas_runtime_depth_pixel_budget >= 1280
  and .first_contact_atlas_readability_guard.manifest_frame_gate == true
  and .first_contact_atlas_readability_guard.family_frame_available_gate == true
  and .first_contact_atlas_readability_guard.atlas_manifest_gate == true
  and .first_contact_atlas_readability_guard.terrain_atlas_frame_gate == true
  and .first_contact_atlas_readability_guard.unit_atlas_sprite_gate == true
  and .first_contact_atlas_readability_guard.structure_atlas_sprite_gate == true
  and .first_contact_atlas_readability_guard.objective_atlas_sprite_gate == true
  and .first_contact_atlas_readability_guard.worker_frame_family_gate == true
  and .first_contact_atlas_readability_guard.scout_frame_family_gate == true
  and .first_contact_atlas_readability_guard.warden_frame_family_gate == true
  and .first_contact_atlas_readability_guard.relay_unit_frame_family_gate == true
  and .first_contact_atlas_readability_guard.command_core_frame_family_gate == true
  and .first_contact_atlas_readability_guard.relay_structure_frame_family_gate == true
  and .first_contact_atlas_readability_guard.beacon_frame_family_gate == true
  and .first_contact_atlas_readability_guard.override_frame_family_gate == true
  and .first_contact_atlas_readability_guard.atlas_family_perimeter_placement_gate == true
  and .first_contact_atlas_readability_guard.atlas_composition_gate == true
  and .first_contact_atlas_readability_guard.atlas_frame_family_gate == true
  and .first_contact_atlas_readability_guard.atlas_runtime_depth_gate == true
  and .first_contact_atlas_readability_guard.secondary_objective_atlas_deemphasis_gate == true
  and .first_contact_atlas_readability_guard.no_copy_boundary_gate == true
  and .first_contact_atlas_readability_guard.first_contact_atlas_readability_gate == true
  and .first_contact_atlas_readability_guard_gate == true
  and .first_contact_visual_hierarchy_contract == "trillionnium_world_bevy_classic_rts_first_contact_visual_hierarchy_v1"
  and .first_contact_visual_hierarchy_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_visual_hierarchy_v1"
  and .first_contact_visual_hierarchy_guard.green == true
  and .first_contact_visual_hierarchy_guard.source_path == "trnm-world-bevy classic_draw_first_contact_visual_hierarchy_layer between readability overlays and selection/combat focus"
  and .first_contact_visual_hierarchy_guard.corridor_tiles == ["14,11","15,11","15,12","15,16","16,9","16,10","17,12"]
  and .first_contact_visual_hierarchy_guard.route_focus_tiles == ["14,11","15,11","16,10","16,9"]
  and .first_contact_visual_hierarchy_guard.target_focus_tile == "16,9"
  and .first_contact_visual_hierarchy_guard.blocked_focus_tile == "15,16"
  and .first_contact_visual_hierarchy_guard.unique_gallery_lanes == ["east_gallery","north_gallery","west_gallery"]
  and .first_contact_visual_hierarchy_guard.atlas_family_busy_core_tiles == []
  and (.first_contact_visual_hierarchy_guard.hierarchy_signatures | index("route_corridor_micro_edge_cues") != null)
  and (.first_contact_visual_hierarchy_guard.hierarchy_signatures | index("route_spine_micro_shadow_cues") != null)
  and (.first_contact_visual_hierarchy_guard.hierarchy_signatures | index("attack_target_micro_backplate") != null)
  and (.first_contact_visual_hierarchy_guard.hierarchy_signatures | index("blocked_warning_micro_backplate") != null)
  and (.first_contact_visual_hierarchy_guard.hierarchy_signatures | index("perimeter_gallery_preserved") != null)
  and .first_contact_visual_hierarchy_guard.corridor_deemphasis_cue_width_px == 8
  and .first_contact_visual_hierarchy_guard.corridor_deemphasis_cue_height_px == 2
  and .first_contact_visual_hierarchy_guard.corridor_deemphasis_cues_per_tile == 2
  and .first_contact_visual_hierarchy_guard.corridor_deemphasis_pixel_budget <= 224
  and .first_contact_visual_hierarchy_guard.route_spine_shadow_cue_width_px == 6
  and .first_contact_visual_hierarchy_guard.route_spine_shadow_cue_height_px == 2
  and .first_contact_visual_hierarchy_guard.route_spine_shadow_pixel_budget <= 120
  and .first_contact_visual_hierarchy_guard.selected_halo_pixel_budget >= 384
  and .first_contact_visual_hierarchy_guard.target_backplate_pixel_budget == 96
  and .first_contact_visual_hierarchy_guard.blocked_backplate_pixel_budget == 72
  and .first_contact_visual_hierarchy_guard.corridor_tile_gate == true
  and .first_contact_visual_hierarchy_guard.route_spine_gate == true
  and .first_contact_visual_hierarchy_guard.selected_halo_gate == true
  and .first_contact_visual_hierarchy_guard.combat_backplate_gate == true
  and .first_contact_visual_hierarchy_guard.gallery_preservation_gate == true
  and .first_contact_visual_hierarchy_guard.hierarchy_signature_gate == true
  and (.first_contact_visual_hierarchy_guard.hierarchy_layer_draw_order | index("visual_hierarchy_deemphasis")) == 14
  and (.first_contact_visual_hierarchy_guard.hierarchy_layer_draw_order | index("central_clarity_deemphasis")) == 15
  and (.first_contact_visual_hierarchy_guard.hierarchy_layer_draw_order | index("terminal_legibility_deemphasis")) == 16
  and (.first_contact_visual_hierarchy_guard.hierarchy_layer_draw_order | index("selection_combat_focus")) == 17
  and .first_contact_visual_hierarchy_guard.hierarchy_layer_order_gate == true
  and .first_contact_visual_hierarchy_guard.visual_hierarchy_gate == true
  and .first_contact_visual_hierarchy_guard_gate == true
  and .first_contact_central_clarity_contract == "trillionnium_world_bevy_classic_rts_first_contact_central_clarity_v1"
  and .first_contact_central_clarity_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_central_clarity_v1"
  and .first_contact_central_clarity_guard.green == true
  and .first_contact_central_clarity_guard.source_path == "trnm-world-bevy classic_draw_first_contact_central_clarity_layer between visual hierarchy and selection/combat focus"
  and .first_contact_central_clarity_guard.central_core_tile_count == 18
  and .first_contact_central_clarity_guard.central_quiet_tile_count == 13
  and .first_contact_central_clarity_guard.central_focus_tile_count == 5
  and .first_contact_central_clarity_guard.quiet_tiles == ["13,10","14,10","15,10","17,10","18,10","13,11","16,11","17,11","18,11","13,12","14,12","16,12","18,12"]
  and .first_contact_central_clarity_guard.focus_corridor_tiles == ["14,11","15,11","15,12","15,16","16,9","16,10","17,12"]
  and .first_contact_central_clarity_guard.focus_overlap_tiles == []
  and .first_contact_central_clarity_guard.central_quiet_cue_width_px == 6
  and .first_contact_central_clarity_guard.central_quiet_cue_height_px == 2
  and .first_contact_central_clarity_guard.central_quiet_cues_per_tile == 2
  and .first_contact_central_clarity_guard.quiet_tile_fill_pixel_budget == 0
  and .first_contact_central_clarity_guard.quiet_tile_pixel_budget <= 312
  and .first_contact_central_clarity_guard.quiet_edge_pixel_budget <= 156
  and (.first_contact_central_clarity_guard.clarity_signatures | index("central_negative_space_micro_edge_cues") != null)
  and (.first_contact_central_clarity_guard.clarity_signatures | index("focus_corridor_not_muted") != null)
  and (.first_contact_central_clarity_guard.clarity_layer_draw_order | index("visual_hierarchy_deemphasis")) == 14
  and (.first_contact_central_clarity_guard.clarity_layer_draw_order | index("central_clarity_deemphasis")) == 15
  and (.first_contact_central_clarity_guard.clarity_layer_draw_order | index("terminal_legibility_deemphasis")) == 16
  and (.first_contact_central_clarity_guard.clarity_layer_draw_order | index("selection_combat_focus")) == 17
  and .first_contact_central_clarity_guard.central_quiet_tile_gate == true
  and .first_contact_central_clarity_guard.focus_overlap_gate == true
  and .first_contact_central_clarity_guard.quiet_edge_gate == true
  and .first_contact_central_clarity_guard.clarity_signature_gate == true
  and .first_contact_central_clarity_guard.clarity_layer_order_gate == true
  and .first_contact_central_clarity_guard.central_clarity_gate == true
  and .first_contact_central_clarity_guard_gate == true
  and .first_contact_terminal_legibility_contract == "trillionnium_world_bevy_classic_rts_first_contact_terminal_legibility_v1"
  and .first_contact_terminal_legibility_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_terminal_legibility_v1"
  and .first_contact_terminal_legibility_guard.green == true
  and .first_contact_terminal_legibility_guard.source_path == "trnm-world-bevy classic_draw_first_contact_terminal_legibility_layer between central clarity and selection/combat focus"
  and .first_contact_terminal_legibility_guard.terminal_quiet_tile_count == 13
  and .first_contact_terminal_legibility_guard.target_quiet_tiles == ["15,8","16,8","17,8","15,9","17,9"]
  and .first_contact_terminal_legibility_guard.blocked_quiet_tiles == ["14,15","15,15","16,15","14,16","16,16","14,17","15,17","16,17"]
  and .first_contact_terminal_legibility_guard.terminal_focus_tiles == ["15,16","16,9","16,10"]
  and .first_contact_terminal_legibility_guard.focus_overlap_tiles == []
  and .first_contact_terminal_legibility_guard.target_focus_tile == "16,9"
  and .first_contact_terminal_legibility_guard.blocked_focus_tile == "15,16"
  and .first_contact_terminal_legibility_guard.terminal_quiet_cue_width_px == 6
  and .first_contact_terminal_legibility_guard.terminal_quiet_cue_height_px == 2
  and .first_contact_terminal_legibility_guard.terminal_quiet_cues_per_tile == 2
  and .first_contact_terminal_legibility_guard.target_quiet_fill_pixel_budget == 0
  and .first_contact_terminal_legibility_guard.blocked_quiet_fill_pixel_budget == 0
  and .first_contact_terminal_legibility_guard.target_quiet_pixel_budget <= 120
  and .first_contact_terminal_legibility_guard.blocked_quiet_pixel_budget <= 192
  and .first_contact_terminal_legibility_guard.target_edge_pixel_budget <= 60
  and .first_contact_terminal_legibility_guard.blocked_edge_pixel_budget <= 96
  and (.first_contact_terminal_legibility_guard.terminal_signatures | index("target_terminal_micro_edge_cues") != null)
  and (.first_contact_terminal_legibility_guard.terminal_signatures | index("blocked_terminal_micro_edge_cues") != null)
  and (.first_contact_terminal_legibility_guard.terminal_signatures | index("route_terminal_focus_preserved") != null)
  and (.first_contact_terminal_legibility_guard.terminal_layer_draw_order | index("central_clarity_deemphasis")) == 15
  and (.first_contact_terminal_legibility_guard.terminal_layer_draw_order | index("terminal_legibility_deemphasis")) == 16
  and (.first_contact_terminal_legibility_guard.terminal_layer_draw_order | index("selection_combat_focus")) == 17
  and .first_contact_terminal_legibility_guard.target_terminal_quiet_gate == true
  and .first_contact_terminal_legibility_guard.blocked_terminal_quiet_gate == true
  and .first_contact_terminal_legibility_guard.terminal_focus_preservation_gate == true
  and .first_contact_terminal_legibility_guard.terminal_edge_budget_gate == true
  and .first_contact_terminal_legibility_guard.terminal_signature_gate == true
  and .first_contact_terminal_legibility_guard.terminal_layer_order_gate == true
  and .first_contact_terminal_legibility_guard.terminal_legibility_gate == true
  and .first_contact_terminal_legibility_guard_gate == true
  and .first_contact_selection_combat_focus_contract == "trillionnium_world_bevy_classic_rts_first_contact_selection_combat_focus_v1"
  and .first_contact_selection_combat_focus_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_selection_combat_focus_v1"
  and .first_contact_selection_combat_focus_guard.green == true
  and .first_contact_selection_combat_focus_guard.source_path == "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer after atlas/motion/readability overlays"
  and .first_contact_selection_combat_focus_guard.selected_focus_tiles == ["14,11","15,11","15,12","17,12"]
  and .first_contact_selection_combat_focus_guard.route_focus_tiles == ["14,11","15,11","16,10","16,9"]
  and .first_contact_selection_combat_focus_guard.route_clearance_tiles == ["13,11","14,10","14,12","15,9","15,10","16,8","16,11","17,9","17,10"]
  and .first_contact_selection_combat_focus_guard.route_clearance_overlap_tiles == []
  and .first_contact_selection_combat_focus_guard.target_focus_tile == "16,9"
  and .first_contact_selection_combat_focus_guard.blocked_focus_tile == "15,16"
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("selected_corner_brackets") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("selected_role_badge_micro_pips") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("compact_route_dashes") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("route_ack_micro_dots") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("route_clearance_corner_cues") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("attack_target_lock_brackets") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("compact_target_lock_cross") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("target_ack_micro_tick") != null)
  and (.first_contact_selection_combat_focus_guard.focus_signatures | index("blocked_warning_cross") != null)
  and .first_contact_selection_combat_focus_guard.route_dash_count >= 4
  and .first_contact_selection_combat_focus_guard.route_ack_tick_count >= 6
  and .first_contact_selection_combat_focus_guard.route_line_step_count >= 10
  and .first_contact_selection_combat_focus_guard.route_dash_width_px == 8
  and .first_contact_selection_combat_focus_guard.route_dash_height_px == 2
  and .first_contact_selection_combat_focus_guard.route_ack_tick_width_px == 2
  and .first_contact_selection_combat_focus_guard.route_ack_tick_height_px == 2
  and .first_contact_selection_combat_focus_guard.selected_role_badge_tick_width_px == 2
  and .first_contact_selection_combat_focus_guard.selected_role_badge_tick_height_px == 2
  and .first_contact_selection_combat_focus_guard.selected_role_badge_tick_pixel_budget == 16
  and .first_contact_selection_combat_focus_guard.selected_focus_pixel_budget <= 288
  and .first_contact_selection_combat_focus_guard.route_ack_tick_pixel_budget == 24
  and .first_contact_selection_combat_focus_guard.route_dash_pixel_budget == 64
  and .first_contact_selection_combat_focus_guard.route_focus_pixel_budget <= 88
  and .first_contact_selection_combat_focus_guard.route_clearance_corner_cue_count == 36
  and .first_contact_selection_combat_focus_guard.route_clearance_corner_cue_width_px == 6
  and .first_contact_selection_combat_focus_guard.route_clearance_corner_cue_height_px == 2
  and .first_contact_selection_combat_focus_guard.route_clearance_corner_cues_per_tile == 4
  and .first_contact_selection_combat_focus_guard.route_clearance_corner_cue_pixel_budget == 432
  and .first_contact_selection_combat_focus_guard.route_clearance_gutter_fill_pixel_budget == 0
  and .first_contact_selection_combat_focus_guard.route_clearance_pixel_budget <= 432
  and .first_contact_selection_combat_focus_guard.combat_target_cross_long_px == 28
  and .first_contact_selection_combat_focus_guard.combat_target_cross_thickness_px == 3
  and .first_contact_selection_combat_focus_guard.combat_target_ack_tick_width_px == 4
  and .first_contact_selection_combat_focus_guard.combat_target_ack_tick_height_px == 2
  and .first_contact_selection_combat_focus_guard.combat_target_cross_pixel_budget <= 168
  and .first_contact_selection_combat_focus_guard.combat_target_ack_tick_pixel_budget == 8
  and .first_contact_selection_combat_focus_guard.combat_target_pixel_budget <= 176
  and .first_contact_selection_combat_focus_guard.blocked_warning_pixel_budget >= 72
  and .first_contact_selection_combat_focus_guard.selected_focus_gate == true
  and .first_contact_selection_combat_focus_guard.route_focus_gate == true
  and .first_contact_selection_combat_focus_guard.route_clearance_gate == true
  and .first_contact_selection_combat_focus_guard.combat_target_focus_gate == true
  and .first_contact_selection_combat_focus_guard.blocked_warning_focus_gate == true
  and .first_contact_selection_combat_focus_guard.focus_signature_gate == true
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("atlas_readability")) == 6
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("readability_overlays")) == 13
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("visual_hierarchy_deemphasis")) == 14
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("central_clarity_deemphasis")) == 15
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("terminal_legibility_deemphasis")) == 16
  and (.first_contact_selection_combat_focus_guard.focus_layer_draw_order | index("selection_combat_focus")) == 17
  and .first_contact_selection_combat_focus_guard.focus_layer_order_gate == true
  and .first_contact_selection_combat_focus_guard.selection_combat_focus_readability_gate == true
  and .first_contact_selection_combat_focus_guard_gate == true
  and .first_contact_target_callout_contract == "trillionnium_world_bevy_classic_rts_first_contact_target_callout_v1"
  and .first_contact_target_callout_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_target_callout_v1"
  and .first_contact_target_callout_guard.green == true
  and .first_contact_target_callout_guard.source_path == "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer compact target callout plate inside final focus layer"
  and .first_contact_target_callout_guard.target_tile == "16,9"
  and .first_contact_target_callout_guard.target_subject == "BEACON"
  and .first_contact_target_callout_guard.target_label == "BEACON 38%"
  and .first_contact_target_callout_guard.target_label_width_px <= 68
  and .first_contact_target_callout_guard.target_health_percent == 38
  and .first_contact_target_callout_guard.target_health_fill_px == 20
  and .first_contact_target_callout_guard.target_callout_width_px == 78
  and .first_contact_target_callout_guard.target_callout_height_px == 20
  and .first_contact_target_callout_guard.target_callout_x_offset_px == 42
  and .first_contact_target_callout_guard.target_callout_y_offset_px == -42
  and .first_contact_target_callout_guard.target_callout_health_bar_width_px == 54
  and .first_contact_target_callout_guard.target_callout_health_bar_height_px == 3
  and .first_contact_target_callout_guard.target_callout_pixel_budget >= 1560
  and .first_contact_target_callout_guard.target_callout_leader_pixel_budget >= 64
  and .first_contact_target_callout_guard.target_callout_clearance_pad_px == 0
  and .first_contact_target_callout_guard.target_callout_leader_clearance_width_px == 0
  and .first_contact_target_callout_guard.target_callout_leader_clearance_height_px == 0
  and .first_contact_target_callout_guard.target_callout_clearance_pixel_budget == 0
  and .first_contact_target_callout_guard.target_prefocus_ring_count == 2
  and .first_contact_target_callout_guard.target_prefocus_ring_thickness_px == 2
  and .first_contact_target_callout_guard.target_prefocus_cross_long_px == 16
  and .first_contact_target_callout_guard.target_prefocus_cross_thickness_px == 2
  and .first_contact_target_callout_guard.target_prefocus_marker_pixel_budget <= 256
  and (.first_contact_target_callout_guard.target_callout_signatures | index("compact_target_callout_plate") != null)
  and (.first_contact_target_callout_guard.target_callout_signatures | index("target_subject_label") != null)
  and (.first_contact_target_callout_guard.target_callout_signatures | index("target_health_strip") != null)
  and (.first_contact_target_callout_guard.target_callout_signatures | index("prefocus_target_rings_capped") != null)
  and (.first_contact_target_callout_guard.target_callout_signatures | index("prefocus_target_cross_thinned") != null)
  and (.first_contact_target_callout_guard.target_callout_signatures | index("target_lock_preserved") != null)
  and (.first_contact_target_callout_guard.target_callout_layer_draw_order | index("terminal_legibility_deemphasis")) == 16
  and (.first_contact_target_callout_guard.target_callout_layer_draw_order | index("selection_combat_focus")) == 17
  and .first_contact_target_callout_guard.target_label_gate == true
  and .first_contact_target_callout_guard.target_health_gate == true
  and .first_contact_target_callout_guard.target_geometry_gate == true
  and .first_contact_target_callout_guard.target_leader_gate == true
  and .first_contact_target_callout_guard.target_callout_clearance_gate == true
  and .first_contact_target_callout_guard.target_prefocus_marker_gate == true
  and .first_contact_target_callout_guard.target_signature_gate == true
  and .first_contact_target_callout_guard.target_layer_order_gate == true
  and .first_contact_target_callout_guard.target_callout_gate == true
  and .first_contact_target_callout_guard_gate == true
  and .first_contact_marker_budget_contract == "trillionnium_world_bevy_classic_rts_first_contact_marker_budget_v1"
  and .first_contact_marker_budget_guard.contract_version == "trillionnium_world_bevy_classic_rts_first_contact_marker_budget_v1"
  and .first_contact_marker_budget_guard.green == true
  and .first_contact_marker_budget_guard.source_path == "trnm-world-bevy muted First Contact atlas gallery presentation plus player-screen tactical/hover marker budgets and final selection/combat focus layer"
  and .first_contact_marker_budget_guard.gallery_sample_count == 14
  and .first_contact_marker_budget_guard.muted_gallery_sample_count == 14
  and .first_contact_marker_budget_guard.busy_core_tiles == []
  and .first_contact_marker_budget_guard.lower_lane_gallery_tiles == ["29,22","29,24","29,26"]
  and .first_contact_marker_budget_guard.west_gallery_frame_count == 4
  and .first_contact_marker_budget_guard.north_gallery_frame_count == 4
  and .first_contact_marker_budget_guard.east_gallery_frame_count == 6
  and .first_contact_marker_budget_guard.max_gallery_lane_frame_count == 6
  and .first_contact_marker_budget_guard.gallery_mute_overlay_pixel_budget >= 21248
  and .first_contact_marker_budget_guard.gallery_slot_cue_pixel_budget <= 14
  and .first_contact_marker_budget_guard.lower_lane_gallery_sample_count == 3
  and .first_contact_marker_budget_guard.lower_lane_rendered_frame_pixel_budget == 0
  and .first_contact_marker_budget_guard.lower_lane_frame_suppressed_count == 3
  and .first_contact_marker_budget_guard.lower_lane_mute_overlay_pixel_budget == 0
  and .first_contact_marker_budget_guard.lower_lane_slot_cue_pixel_budget <= 3
  and .first_contact_marker_budget_guard.lower_lane_ghost_anchor_count == 3
  and .first_contact_marker_budget_guard.lower_lane_gallery_darken_numerator == 5
  and .first_contact_marker_budget_guard.lower_lane_gallery_darken_denominator == 6
  and .first_contact_marker_budget_guard.lower_lane_dim_silhouette_pixel_budget == 0
  and .first_contact_marker_budget_guard.lower_lane_shadow_suppressed_count == 3
  and .first_contact_marker_budget_guard.gallery_hot_marker_color_count == 0
  and .first_contact_marker_budget_guard.lower_lane_hot_marker_color_count == 0
  and .first_contact_marker_budget_guard.interactive_hot_marker_role_count >= 5
  and .first_contact_marker_budget_guard.tactical_track_count == 6
  and .first_contact_marker_budget_guard.player_screen_tactical_track_count == 1
  and .first_contact_marker_budget_guard.player_screen_primary_tactical_track_count == 1
  and .first_contact_marker_budget_guard.player_screen_secondary_tactical_track_count == 0
  and .first_contact_marker_budget_guard.hover_route_path_marker_count == 8
  and .first_contact_marker_budget_guard.player_screen_hover_route_path_marker_count == 0
  and .first_contact_marker_budget_guard.non_focus_owner_identity_colors == ["457953","6a5e4b"]
  and .first_contact_marker_budget_guard.non_focus_owner_identity_hot_color_count == 0
  and .first_contact_marker_budget_guard.non_focus_owner_identity_pixel_budget <= 192
  and .first_contact_marker_budget_guard.player_screen_relay_identity_rail_count == 6
  and .first_contact_marker_budget_guard.player_screen_relay_identity_rail_width_px == 16
  and .first_contact_marker_budget_guard.player_screen_relay_identity_rail_height_px == 2
  and .first_contact_marker_budget_guard.player_screen_relay_identity_rail_pixel_budget <= 192
  and .first_contact_marker_budget_guard.player_screen_legacy_midfield_status_bar_pixel_budget == 0
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_count == 3
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_cues_per_beacon == 6
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_cue_count == 18
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_cue_width_px == 6
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_cue_height_px == 2
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_pixel_budget <= 216
  and .first_contact_marker_budget_guard.selected_role_badge_tick_pixel_budget == 16
  and .first_contact_marker_budget_guard.interactive_focus_pixel_budget == 636
  and .first_contact_marker_budget_guard.selected_focus_tiles == ["14,11","15,11","15,12","17,12"]
  and .first_contact_marker_budget_guard.route_focus_tiles == ["14,11","15,11","16,10","16,9"]
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("darkened_gallery_frames") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("compact_route_dashes") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("perimeter_gallery_edge_anchors") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("perimeter_gallery_single_pixel_anchors") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_gallery_deemphasis") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_micro_slot_cues") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_frame_suppressed") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_anchor_only") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_shadow_suppressed") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_ghost_anchors") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("lower_lane_single_point_ghost_anchors") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("route_ack_micro_dots") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("interactive_focus_kept_hot") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("non_focus_owner_identity_muted") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("player_screen_primary_tactical_track_only") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("player_screen_hover_route_path_suppressed") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("player_screen_relay_identity_micro_rails") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("player_screen_legacy_midfield_status_bars_suppressed") != null)
  and (.first_contact_marker_budget_guard.gallery_presentation_signatures | index("player_screen_secondary_beacon_micro_markers") != null)
  and .first_contact_marker_budget_guard.marker_budget_layer_draw_order == ["atlas_readability","atlas_gallery_muted","visual_hierarchy_deemphasis","central_clarity_deemphasis","terminal_legibility_deemphasis","selection_combat_focus"]
  and .first_contact_marker_budget_guard.gallery_lane_budget_gate == true
  and .first_contact_marker_budget_guard.gallery_mute_gate == true
  and .first_contact_marker_budget_guard.lower_lane_gallery_deemphasis_gate == true
  and .first_contact_marker_budget_guard.interactive_focus_preservation_gate == true
  and .first_contact_marker_budget_guard.non_focus_owner_identity_gate == true
  and .first_contact_marker_budget_guard.player_screen_tactical_track_gate == true
  and .first_contact_marker_budget_guard.player_screen_hover_route_path_gate == true
  and .first_contact_marker_budget_guard.player_screen_relay_identity_rail_gate == true
  and .first_contact_marker_budget_guard.player_screen_legacy_midfield_status_bar_gate == true
  and .first_contact_marker_budget_guard.player_screen_secondary_beacon_marker_gate == true
  and .first_contact_marker_budget_guard.marker_budget_layer_order_gate == true
  and .first_contact_marker_budget_guard.first_contact_marker_budget_gate == true
  and .first_contact_marker_budget_guard_gate == true
  and .first_contact_runtime_core_visibility.green == true
  and .first_contact_runtime_core_visibility.source_path == "trnm-world-bevy classic_draw_first_contact_runtime_core_layer player-visible actor subset with control and semantic fixtures hidden, inline health bars decluttered, default selection rings suppressed, and unit role accents muted"
  and .first_contact_runtime_core_visibility.runtime_core_visible_tile_y_max == 28
  and .first_contact_runtime_core_visibility.runtime_core_actor_count == 83
  and .first_contact_runtime_core_visibility.runtime_core_visible_actor_count == 14
  and .first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_count == 4
  and .first_contact_runtime_core_visibility.runtime_core_suppressed_inline_health_bar_actor_count == 10
  and .first_contact_runtime_core_visibility.runtime_core_progress_bar_actor_count == 2
  and .first_contact_runtime_core_visibility.runtime_core_selection_ring_candidate_actor_count == 14
  and .first_contact_runtime_core_visibility.runtime_core_selection_ring_visible_actor_count == 0
  and .first_contact_runtime_core_visibility.runtime_core_suppressed_selection_ring_actor_count == 14
  and .first_contact_runtime_core_visibility.runtime_core_unit_role_accent_candidate_actor_count == 9
  and .first_contact_runtime_core_visibility.runtime_core_unit_role_accent_visible_actor_count == 0
  and .first_contact_runtime_core_visibility.runtime_core_suppressed_unit_role_accent_actor_count == 9
  and .first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_count == 69
  and .first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_count == 48
  and .first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_count == 20
  and .first_contact_runtime_core_visibility.runtime_core_required_visible_actor_ids == ["multi0.command.core","multi0.worker.0","multi0.flux.relay","map.actor15"]
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.damaged.relay") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.line.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.scout.intel") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.warden.capture") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.veteran.warden") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.command.core") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("multi0.worker.0") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_inline_health_bar_actor_ids | index("map.actor15") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_progress_bar_actor_ids | index("map.actor15") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_progress_bar_actor_ids | index("multi0.worker.1") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_selection_ring_candidate_actor_ids | index("multi0.worker.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_selection_ring_candidate_actor_ids | index("multi0.line.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_selection_ring_visible_actor_ids | length == 0)
  and (.first_contact_runtime_core_visibility.runtime_core_selection_ring_visible_actor_ids | index("multi0.worker.0") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_unit_role_accent_candidate_actor_ids | index("multi0.worker.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_unit_role_accent_candidate_actor_ids | index("multi0.scout.intel") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_unit_role_accent_candidate_actor_ids | index("multi0.striker.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_unit_role_accent_candidate_actor_ids | index("multi0.warden.capture") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_unit_role_accent_visible_actor_ids | length == 0)
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_actor_ids == ["multi0.command.core"]
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_ticks_per_actor == 4
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_tick_width_px == 6
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_tick_height_px == 2
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_pixel_budget <= 48
  and .first_contact_runtime_core_visibility.runtime_core_command_core_hot_body_pixel_budget == 0
  and (.first_contact_runtime_core_visibility.runtime_core_command_core_body_signatures | index("player_screen_runtime_command_core_body_muted") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_command_core_body_signatures | index("player_screen_runtime_command_core_micro_ticks") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.command.core") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.worker.0") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.flux.relay") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("map.actor15") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.formation.lead") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.append.seed") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.recall.order.old.seed") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.queue.reject.runner") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.override.runner") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.obstruction.leader") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.reservation.lead") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.traffic.stuck.runner") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.focus.warden.a") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.priority.guard") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_visible_actor_ids | index("multi0.attackmove.warden") == null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.formation.lead") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.obstruction.leader") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.reservation.lead") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.traffic.stuck.runner") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.obstruction.leader") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.reservation.lead") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.traffic.stuck.runner") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.focus.warden.a") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.priority.guard") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_semantic_fixture_actor_ids | index("multi0.attackmove.warden") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_ids | index("multi0.append.seed") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_ids | index("multi0.remove.seed") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_ids | index("multi0.recall.order.old.seed") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_ids | index("multi0.recall.override.old.seed") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.queue.reject.runner") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_ids | index("multi0.override.runner") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_tile_ids | index("17,25") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_tile_ids | index("28,27") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_tile_ids | index("18,31") != null)
  and (.first_contact_runtime_core_visibility.runtime_core_hidden_fixture_tile_ids | index("31,31") != null)
  and .first_contact_runtime_core_visibility.runtime_core_required_visible_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_hidden_fixture_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_control_fixture_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_semantic_fixture_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_visible_subset_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_bottom_fixture_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_inline_health_bar_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_selection_ring_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_unit_role_accent_gate == true
  and .first_contact_runtime_core_visibility.runtime_core_command_core_body_gate == true
  and .first_contact_runtime_core_visibility_gate == true
  and .rts_data_player_screen_chrome_profile.group_summary_prefix == "GROUP"
  and .rts_data_player_screen_chrome_profile.group_summary_suffix == "UNITS SELECTED"
  and .rts_data_player_screen_chrome_profile.production_slot_visible_count == 4
  and .rts_data_player_screen_chrome_profile.production_slot_column_count == 2
  and .rts_data_player_screen_chrome_profile.build_palette_visible_count == 8
  and .rts_data_player_screen_chrome_profile.build_palette_column_count == 4
  and .rts_data_player_screen_chrome_gate == true
  and .rts_data_player_screen_profile.camera_focus_tile.x == 16
  and .rts_data_player_screen_profile.camera_focus_tile.y == 16
  and .rts_data_player_screen_profile.command_destination_tile.x == 16
  and .rts_data_player_screen_profile.command_destination_tile.y == 9
  and (.rts_data_player_screen_profile.command_queue | length) == 4
  and (.rts_data_player_screen_profile.command_queue | index("build:trnm.flux.relay") != null)
  and (.rts_data_player_screen_profile.command_queue | index("train:trnm.worker") != null)
  and (.rts_data_player_screen_profile.command_queue | index("attack:trnm.flux.beacon") != null)
  and (.rts_data_player_screen_profile.production_queue | length) == 3
  and (.rts_data_player_screen_profile.production_queue | index("train:guard") != null)
  and (.rts_data_player_screen_profile.production_queue | index("upgrade:signal_blade") != null)
  and (.rts_data_player_screen_profile.build_queue | length) == 2
  and (.rts_data_player_screen_profile.build_queue | index("build:watch_tower") != null)
  and (.rts_data_player_screen_profile.build_queue | index("upgrade:training_hall") != null)
  and .rts_data_player_screen_profile.unit_health_percents == [96,78,71,34]
  and .rts_data_player_screen_profile.active_ability_id == "worker"
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("worker") != null)
  and .rts_data_player_screen_profile.ability_cooldown_percents == [0,0,16,0,42,25]
  and (.rts_data_player_screen_profile.ability_cooldown_percents | length) == (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | length)
  and (.rts_data_player_screen_profile.visible_tiles | length) == 64
  and (.rts_data_player_screen_profile.fogged_tiles | length) == 6
  and .rts_data_player_screen_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.contract_version == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.map_id == "first_contact_basin"
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.room_id == "first-contact-basin"
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.layout.player_map.map_origin_x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.chrome.top_title == "TRNM RTS"
  and (.rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.chrome.command_grid_slot_ids | index("worker") != null)
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.command_queue == .rts_data_player_screen_profile.command_queue
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_layout_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_chrome_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile_gate == true
  and .rts_data_player_screen_profile == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile
  and .rts_data_player_screen_layout_profile == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.layout
  and .rts_data_player_screen_chrome_profile == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.chrome
  and .rts_data_player_screen_layout_gate == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_layout_gate
  and .rts_data_player_screen_chrome_gate == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_chrome_gate
  and .rts_data_player_screen_gate == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile_gate
  and .rts_bevy_runtime_player_screen_application_contract == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_bevy_runtime_player_screen_application.contract_version == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_bevy_runtime_player_screen_application.green == true
  and .rts_bevy_runtime_player_screen_application.profile_contract == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_bevy_runtime_player_screen_application.map_scene == "first_contact_basin"
  and .rts_bevy_runtime_player_screen_application.current_room_id == "first-contact-basin"
  and .rts_bevy_runtime_player_screen_application.camera_focus_tile_id == "16,16"
  and .rts_bevy_runtime_player_screen_application.command_destination_tile_id == "16,9"
  and .rts_bevy_runtime_player_screen_application.command_queue == .rts_data_player_screen_profile.command_queue
  and .runtime_player_screen_command_queue_count == (.rts_bevy_runtime_player_screen_application.command_queue | length)
  and .runtime_player_screen_command_queue_count == 4
  and .rts_bevy_runtime_player_screen_application.production_queue == .rts_data_player_screen_profile.production_queue
  and .runtime_player_screen_production_queue_count == (.rts_bevy_runtime_player_screen_application.production_queue | length)
  and .runtime_player_screen_production_queue_count == 3
  and .rts_bevy_runtime_player_screen_application.build_queue == .rts_data_player_screen_profile.build_queue
  and .runtime_player_screen_build_queue_count == (.rts_bevy_runtime_player_screen_application.build_queue | length)
  and .runtime_player_screen_build_queue_count == 2
  and .rts_bevy_runtime_player_screen_application.ability_command_ids == .rts_data_player_screen_chrome_profile.command_grid_slot_ids
  and .runtime_player_screen_ability_command_count == (.rts_bevy_runtime_player_screen_application.ability_command_ids | length)
  and .runtime_player_screen_ability_command_count == 6
  and (.rts_bevy_runtime_player_screen_application.visible_tile_ids | length) == 64
  and .runtime_player_screen_visible_tile_count == (.rts_bevy_runtime_player_screen_application.visible_tile_ids | length)
  and .runtime_player_screen_visible_tile_count == 64
  and .runtime_player_screen_fogged_tile_count == (.rts_bevy_runtime_player_screen_application.fogged_tile_ids | length)
  and .runtime_player_screen_fogged_tile_count == 6
  and (.rts_bevy_runtime_player_screen_application.group_route_tile_ids | index("16,9") != null)
  and .rts_bevy_runtime_player_screen_application.profile_application_gate == true
  and .rts_bevy_runtime_player_screen_application.command_surface_seed_gate == true
  and .rts_bevy_runtime_player_screen_application.route_surface_seed_gate == true
  and .rts_bevy_runtime_player_screen_application.runtime_application_path == "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation"
  and (.rts_bevy_runtime_player_screen_application.source_of_truth | contains("trnm-rts-data First Contact player-screen profile"))
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_evidence_contract == "trnm_rts_evidence_v1"
  and .rts_evidence_bevy_runtime_adapter.contract_version == "trnm_rts_evidence_bevy_runtime_adapter_v1"
  and .rts_evidence_bevy_runtime_adapter.runtime_contract == "trnm_rts_bevy_runtime_adapter_v1"
  and .rts_evidence_bevy_runtime_adapter.green == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_fixture.contract_version == "trnm_rts_online_first_contact_fixture_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_fixture.green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_fixture.envelope.map_id == "first_contact_basin"
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff.contract_version == "trnm_rts_online_local_handoff_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff.green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff.accepted_order_count == 1
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter.contract_version == "trnm_rts_online_offline_adapter_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter.green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter.local_runtime_handoff.accepted_runtime_command_labels == ["move:8,4"]
  and .rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_count == 1156
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples.center.height == 2
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples.resource_zone.resource_zone == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.renderable_tile_count == 1024
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.lane_tile_count == 240
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection.minimap_anchor_actor_count == 39
  and .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection_gate == true
  and .rts_data_terrain_profile_count == .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_count
  and .rts_data_terrain_profile_samples == .rts_evidence_bevy_runtime_adapter.first_contact_terrain_profile_samples
  and .rts_data_renderer_projection == .rts_evidence_bevy_runtime_adapter.first_contact_renderer_projection
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.cell_w == 28
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.x == 464
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample.surface_seed == 12
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate == true
  and .rts_bevy_runtime_map_projection == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection
  and .rts_bevy_runtime_tile_rect_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample
  and .rts_bevy_runtime_terrain_seed_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample
  and .rts_bevy_runtime_map_projection_gate == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate
  and .rts_online_protocol_fixture == .rts_evidence_bevy_runtime_adapter.first_contact_online_protocol_fixture
  and .rts_online_local_handoff == .rts_evidence_bevy_runtime_adapter.first_contact_online_local_handoff
  and .rts_online_offline_adapter == .rts_evidence_bevy_runtime_adapter.first_contact_online_offline_adapter
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_application_contract == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_application_green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_application.contract_version == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_application.green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_application_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_application_green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_application.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_application.command_queue == ["move:8,4"]
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_review.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_review.runtime_command_stamp_tile_id == "8,4"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_review.runtime_application.command_queue == ["move:8,4"]
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_session_transition_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_session_transition_green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_session_transition_review.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_session_transition_review.after_command_queue == ["move:8,4"]
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_green == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_review.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
  and (.rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_review.ready_state_labels | index("authority:offline_loopback:no_socket") != null)
  and .rts_online_offline_adapter_consumption == .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_consumption_review
  and .rts_online_offline_adapter_session_transition == .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_session_transition_review
  and .rts_online_offline_adapter_lobby_ready == .rts_evidence_bevy_runtime_adapter.first_contact_offline_adapter_lobby_ready_review
  and (.rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1") != null)
  and (.rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_before_command_queue_sample | index("build:trnm.flux.relay") != null)
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_after_command_queue_sample == ["move:8,4"]
  and (.rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_ready_state_labels_sample | index("authority:offline_loopback:no_socket") != null)
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_command_stamp_tile_sample == "8,4"
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_gate == true
  and .rts_bevy_runtime_adapter_contract == "trnm_rts_bevy_runtime_adapter_v1"
  and .rts_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_minimap_cell_sample.x == 134
  and .rts_bevy_runtime_minimap_cell_sample.y == 175
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_first_focus_tile_sample == {"x":9,"y":7}
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_minimap_jump_tile_sample == "minimap_cursor_jump"
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_bounds_clamped_sample == true
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_viewport_rect_sample == {"x":19,"y":8,"width":33,"height":19}
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_selection_follow_tile_sample == "mirror_captain"
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_revealed_union_count_sample == 35
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_zoom_rect_area_sample == 308
  and .rts_bevy_runtime_path_preview_sample == "queue_stack"
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_action_kinds_sample == ["select-control-group","move","move","attack","queue","queue"]
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_action_payloads_sample == ["box:frontline","8,4:line","9,2:rally","arena_creep_attack","build:watch_tower@7,4","cancel:build:0"]
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_history_entries_sample == ["command_queue_path_preview:queue_stack","command_queue_path_preview:shift_waypoints","command_queue_path_preview:rally_chain","command_queue_path_preview:attack_focus","command_queue_path_preview:build_reservation","command_queue_path_preview:cancel_repath"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_stage_sample == "commit_spacing"
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_action_payloads_sample == ["box:frontline","8,4:wedge","8,4:line","8,4:wedge","6,5:split","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_history_entries_sample == ["formation_move_preview:destination_ghost","formation_move_preview:wedge_spacing","formation_move_preview:line_reflow","formation_move_preview:collision_avoidance","formation_move_preview:split_avoidance","formation_move_preview:commit_spacing"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_destination_slots_sample == ["8,4","7,4","8,5","9,4"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_split_route_sample == ["5,5","6,4","6,5","7,5","6,6"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_stage_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_action_payloads_sample == ["28","1,31:line","1,31:line","1,31:line"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_history_entries_sample == ["control_group_recall_formation_preview:recall_focus_hud","control_group_recall_formation_preview:formation_anchor_slots","control_group_recall_formation_preview:queued_valid_members","control_group_recall_formation_preview:filtered_invalid"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_slot_tiles_sample == ["1,31","2,31"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_filtered_members_sample == ["missing:multi0.recall.formation.missing","foreign:map.actor1"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_stage_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_action_payloads_sample == ["26","18,31:line","27","20,30:line"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_history_entries_sample == ["control_group_recall_override_preview:group_26_recall_focus","control_group_recall_override_preview:group_26_queued_order","control_group_recall_override_preview:group_27_override_cancel","control_group_recall_override_preview:group_27_final_filtered"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_final_tiles_sample == ["20,30","22,30"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_canceled_members_sample == ["multi0.recall.override.runner","multi0.recall.override.wing"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_stage_sample == "arrival_lock"
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_stage_names_sample == ["slot_claim","path_reservation","stagger_step","crowd_avoidance","blocked_reroute","arrival_lock"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_action_payloads_sample == ["box:frontline","8,4:wedge","8,4:line","6,5:split","8,4:wedge","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_arrival_route_sample == ["6,5","7,5","8,5","9,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_stage_sample == "flow_resume"
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_stage_names_sample == ["detect_block","hold_queue","side_step","gap_claim","flow_resume"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_action_payloads_sample == ["8,4:wedge","8,4:line","6,5:split","box:frontline","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_blocked_tiles_sample == ["7,4","7,5"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_resume_route_sample == ["6,5","7,5","8,5","9,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.npc_behavior_stage_sample == "creep_retreat"
  and .rts_evidence_bevy_runtime_adapter.combat_impact_stage_sample == "damage_tick"
  and .rts_evidence_bevy_runtime_adapter.locomotion_blend_stage_sample == "formation_slide"
  and .rts_evidence_bevy_runtime_adapter.npc_transition_stage_sample == "hit_recover"
  and .rts_evidence_bevy_runtime_adapter.depth_readability_stage_sample == "target_priority"
  and .rts_evidence_bevy_runtime_adapter.structure_modeling_stage_sample == "repair_beam"
  and .rts_evidence_bevy_runtime_adapter.environment_life_stage_sample == "resource_glint"
  and .rts_evidence_bevy_runtime_adapter.worker_harvest_animation_stage_sample == "return_path"
  and .rts_evidence_bevy_runtime_adapter.production_spawn_animation_stage_sample == "supply_flash"
  and .rts_evidence_bevy_runtime_adapter.action_cadence_attack_mark_count_sample == 22
  and .rts_evidence_bevy_runtime_adapter.action_cadence_carry_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.action_cadence_idle_mark_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.action_cadence_creep_windup_offset_sample == -24
  and .rts_evidence_bevy_runtime_adapter.action_sequence_phase_sample == "recovery"
  and .rts_evidence_bevy_runtime_adapter.action_sequence_windup_mark_count_sample == 9
  and .rts_evidence_bevy_runtime_adapter.action_sequence_strike_mark_count_sample == 12
  and .rts_evidence_bevy_runtime_adapter.action_sequence_carry_down_mark_count_sample == 5
  and .rts_evidence_bevy_runtime_adapter.action_sequence_idle_mark_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_guard_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_worker_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_creep_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_creep_role_prop_count_sample == 2
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_face_shade_offset_sample == -32
  and .rts_evidence_bevy_runtime_adapter.command_surface_stage_sample == "target_queue"
  and .rts_bevy_runtime_command_grid_hit_sample == 0
  and (.rts_evidence_bevy_runtime_adapter.tile_line_sample | length) == 9
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].step_index == 0
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].tile_x == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].tile_y == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].step_index == 4
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].tile_x == 10
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].tile_y == 12
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].step_index == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].tile_x == 12
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].tile_y == 16
  and .rts_evidence_bevy_runtime_adapter.combat_engagement_tiles_sample == ["9,3","10,3","10,2","11,2"]
  and .rts_evidence_bevy_runtime_adapter.combat_flash_tiles_sample == ["6,5","6,4"]
  and .rts_evidence_bevy_runtime_adapter.combat_target_tile_sample.x == 9
  and .rts_evidence_bevy_runtime_adapter.combat_target_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.combat_target_priority_sample == ["arena_creep_attack","arena_guard_support","arena_worker_support"]
  and .rts_evidence_bevy_runtime_adapter.combat_projectile_trail_sample == ["5,5","6,5","7,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.combat_ability_effect_tiles_sample == ["10,3","10,2","11,2","9,3"]
  and .rts_evidence_bevy_runtime_adapter.combat_threat_levels_sample == [88,66,41]
  and .rts_evidence_bevy_runtime_adapter.combat_damage_ticks_sample == [16,21,35]
  and .rts_evidence_bevy_runtime_adapter.combat_projectile_id_sample == "guard_break_bolt"
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_wave_units_sample == ["lane_scout","mirror_raider","siege_runner"]
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_tiles_sample == ["9,3","8,4","7,4","6,5"]
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_counter_tiles_sample == ["5,5","6,5","6,4","7,5"]
  and .rts_evidence_bevy_runtime_adapter.enemy_pressure_wave_units_sample == ["enemy_raider","enemy_signal_guard","enemy_sapper"]
  and .rts_evidence_bevy_runtime_adapter.enemy_pressure_lane_tiles_sample == ["10,2","9,3","8,4","7,4","6,5"]
  and .rts_evidence_bevy_runtime_adapter.recon_scout_route_tiles_sample == ["5,5","6,4","7,4","8,3","9,2","10,2"]
  and .rts_evidence_bevy_runtime_adapter.recon_fog_reveal_tiles_sample == ["7,4","8,3","8,2","9,2","9,3","10,2","10,3","11,1","11,2"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structures_sample == ["enemy_watch_post","enemy_barracks","enemy_resource_vault"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_units_sample == ["enemy_scout","enemy_worker","enemy_guard"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structure_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structure_tile_sample.y == 2
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_unit_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_unit_tile_sample.y == 2
  and .rts_evidence_bevy_runtime_adapter.base_assault_path_tiles_sample == ["5,5","6,5","7,4","8,4","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.base_assault_targets_sample == ["enemy_watch_post","enemy_barracks","enemy_resource_vault"]
  and .rts_evidence_bevy_runtime_adapter.aftermath_debris_tiles_sample == ["9,3","10,3","10,4","11,3"]
  and .rts_evidence_bevy_runtime_adapter.aftermath_smoke_tiles_sample == ["10,2","10,3","11,3"]
  and .rts_evidence_bevy_runtime_adapter.commander_aura_tiles_sample == ["6,5","7,4","8,4","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.commander_loot_items_sample == ["barracks_map_cache","field_banner_relic","repair_kit_crate"]
  and .rts_evidence_bevy_runtime_adapter.expansion_claim_tiles_sample == ["8,2","9,2","10,2","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.expansion_structure_tile_sample.x == 8
  and .rts_evidence_bevy_runtime_adapter.expansion_structure_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.expansion_workers_sample == ["expansion_worker_alpha","expansion_worker_beta","expansion_worker_gamma"]
  and .rts_evidence_bevy_runtime_adapter.counterattack_units_sample == ["counter_raider_alpha","counter_raider_beta","counter_sapper"]
  and .rts_evidence_bevy_runtime_adapter.counterattack_route_tiles_sample == ["11,2","10,2","9,3","8,3","7,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.army_units_sample == ["relay_guard_alpha","relay_guard_beta","wayfinder_scout","field_mender"]
  and .rts_evidence_bevy_runtime_adapter.army_rally_tiles_sample == ["5,5","6,5","7,4","8,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.player_army_unit_tile_sample.x == 6
  and .rts_evidence_bevy_runtime_adapter.player_army_unit_tile_sample.y == 4
  and .rts_evidence_bevy_runtime_adapter.central_keep_route_tiles_sample == ["12,3","12,4","13,4","13,3","14,3"]
  and .rts_evidence_bevy_runtime_adapter.central_keep_tile_sample.x == 13
  and .rts_evidence_bevy_runtime_adapter.central_keep_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.boss_guard_units_sample == ["keep_warden_alpha","keep_warden_beta","ward_sentinel"]
  and .rts_evidence_bevy_runtime_adapter.player_siege_line_tiles_sample == ["11,4","12,4","13,4","12,3"]
  and .rts_evidence_bevy_runtime_adapter.keep_breach_tiles_sample == ["13,3","13,4","14,3","14,4"]
  and .rts_evidence_bevy_runtime_adapter.guardian_counter_units_sample == ["high_warden","ward_lancer","last_mirror_guard"]
  and .rts_evidence_bevy_runtime_adapter.keep_claim_tiles_sample == ["12,3","13,3","14,3","13,4"]
  and .rts_evidence_bevy_runtime_adapter.objective_tiles_sample == ["6,5","6,4","7,5","9,2"]
  and .rts_evidence_bevy_runtime_adapter.creep_camp_tiles_sample == ["8,3","8,2","9,3","9,2"]
  and .rts_evidence_bevy_runtime_adapter.terrain_route_tiles_sample == ["5,5","6,5","7,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.terrain_choke_tiles_sample == ["7,4","7,3","8,4"]
  and .rts_evidence_bevy_runtime_adapter.expansion_tiles_sample == ["9,2","10,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.siege_units_sample == ["stonebreak_cart"]
  and .rts_evidence_bevy_runtime_adapter.siege_push_route_tiles_sample == ["9,2","9,3","10,3","10,2","11,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.siege_breach_tiles_sample == ["9,3","10,3","10,2","11,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.enemy_fortification_tile_sample.x == 10
  and .rts_evidence_bevy_runtime_adapter.enemy_fortification_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.enemy_repair_units_sample == ["repair_adept_alpha","repair_adept_beta"]
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_units_sample == ["ridge_sentry_left","ridge_sentry_right","ridge_sapper"]
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_tile_sample.x == 8
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_tile_sample.y == 4
  and .rts_evidence_bevy_runtime_adapter.player_hold_tiles_sample == ["8,3","9,3","9,4","10,3"]
  and .rts_evidence_bevy_runtime_adapter.inner_lane_tiles_sample == ["10,3","11,2","11,3","12,3","12,4"]
  and .rts_evidence_bevy_runtime_adapter.inner_gate_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.inner_gate_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.signal_lock_tile_sample.x == 12
  and .rts_evidence_bevy_runtime_adapter.signal_lock_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.inner_defenders_sample == ["inner_guard_alpha","inner_guard_beta","signal_lancer"]
  and .rts_evidence_bevy_runtime_adapter.supply_convoy_sample == ["convoy_cart","field_medic","ammo_runner"]
  and .rts_evidence_bevy_runtime_adapter.split_squad_tiles_sample == ["10,4","11,4","12,4","12,3"]
  and .rts_evidence_bevy_runtime_adapter.inner_core_tile_sample.x == 12
  and .rts_evidence_bevy_runtime_adapter.inner_core_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.restored_zones_sample == ["central_keep","signal_core","inner_lane","forest_relay"]
  and .rts_evidence_bevy_runtime_adapter.rebuild_structures_sample == ["signal_core","inner_latch","mirror_ward"]
  and .rts_evidence_bevy_runtime_adapter.garrison_units_sample == ["mirror_guard_alpha","signal_lancer","field_engineer"]
  and .rts_evidence_bevy_runtime_adapter.open_world_route_tiles_sample == ["13,3","12,3","11,3","10,2","9,2"]
  and .rts_evidence_bevy_runtime_adapter.open_world_panels_sample == ["room_panel:league-coliseum","task_panel:task-fixture-first-route","combat_panel:league-coliseum","save_panel:post_rts_restore"]
  and .rts_evidence_bevy_runtime_adapter.siege_unit_tile_sample.x == 9
  and .rts_evidence_bevy_runtime_adapter.siege_unit_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.harvest_tile_sample.x == 3
  and .rts_evidence_bevy_runtime_adapter.harvest_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.dropoff_tile_sample.x == 5
  and .rts_evidence_bevy_runtime_adapter.dropoff_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.build_site_tiles_sample == ["7,4","7,5","8,4"]
  and .rts_evidence_bevy_runtime_adapter.structure_tile_sample.x == 4
  and .rts_evidence_bevy_runtime_adapter.structure_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.unlock_unit_tile_sample.x == 7
  and .rts_evidence_bevy_runtime_adapter.unlock_unit_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.queue_gold_cost_sample == 210
  and .rts_evidence_bevy_runtime_adapter.queue_available_gold_sample == 40
  and .rts_evidence_bevy_runtime_adapter.queue_affordable_sample == false
  and .rts_evidence_bevy_runtime_adapter.queue_build_parts_sample == ["watch_tower","7,4"]
  and .rts_evidence_bevy_runtime_adapter.queue_production_lane_sample == true
  and .rts_evidence_bevy_runtime_adapter.queue_feedback_chip_sample == "feedback:build_placed:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.blocked_feedback_chip_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.queue_blocked_feedback_label_sample == "QUEUE LOCK NEED 210G"
  and .rts_evidence_bevy_runtime_adapter.command_panel_slot_id_sample == "attack"
  and .rts_evidence_bevy_runtime_adapter.command_panel_build_palette_queue_id_sample == "build:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.command_panel_production_slot_queue_id_sample == "build:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_cancel_queue_id_sample == "cancel:build:0"
  and .rts_evidence_bevy_runtime_adapter.command_panel_palette_cancel_queue_id_sample == "cancel:active_build"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_slot_status_label_sample == "B1 66 R"
  and .rts_evidence_bevy_runtime_adapter.command_panel_palette_state_label_sample == "ACTIVE"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_queue_summary_sample == "WORKER 42% TOWER 66%"
  and .rts_evidence_bevy_runtime_adapter.command_panel_spawned_unit_id_sample == "worker_3"
  and .rts_evidence_bevy_runtime_adapter.command_panel_structure_id_sample == "watch_tower"
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_pauses_queue_tick_sample == true
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_from_frame_sample == 4
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_id_sample == "cancel_refund"
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_title_sample == "WORKER QUEUED"
  and .rts_evidence_bevy_runtime_adapter.selection_default_units_sample == ["player", "square_guard_patrol", "square_worker_carry", "square_creep_wander"]
  and .rts_evidence_bevy_runtime_adapter.selection_same_class_units_sample == ["player", "square_guard_front", "square_guard_patrol"]
  and .rts_evidence_bevy_runtime_adapter.selection_guard_tile_sample.x == 7
  and .rts_evidence_bevy_runtime_adapter.selection_guard_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.selection_drag_units_sample == ["player", "square_guard_front", "square_guard_patrol", "square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.selection_drag_rejected_units_sample == ["square_creep_wander"]
  and .rts_evidence_bevy_runtime_adapter.selection_drag_distance_sq_sample == 107300
  and .rts_evidence_bevy_runtime_adapter.selection_drag_ready_sample == true
  and .rts_evidence_bevy_runtime_adapter.selection_drag_group_id_sample == "drag:4,4->8,5"
  and .rts_evidence_bevy_runtime_adapter.selection_drag_player_label_sample == "DRAG SELECT 5 UNITS 4,4->8,5"
  and .rts_evidence_bevy_runtime_adapter.selection_tiles_for_units_sample == ["5,4", "4,5"]
  and .rts_evidence_bevy_runtime_adapter.control_group_hotkey_slot_sample == "10"
  and .rts_evidence_bevy_runtime_adapter.control_group_default_slot_three_units_sample == ["square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.control_group_assignment_units_sample == ["square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.slot == "10"
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.key_label == "0"
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.member_count == 2
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.occupied == true
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.active == true
  and .rts_evidence_bevy_runtime_adapter.control_group_merged_units_sample == ["player", "square_worker_carry"]
  and .rts_evidence_bevy_runtime_adapter.selection_clear_parts_sample == ["hostile", "square_creep_wander", "9,4"]
  and .rts_evidence_bevy_runtime_adapter.move_command_parts_sample == ["9,2", "attack_move"]
  and .rts_evidence_bevy_runtime_adapter.line_path_tiles_sample == ["6,5", "7,4", "8,3"]
  and .rts_evidence_bevy_runtime_adapter.focus_fire_units_sample == ["relay_guard_alpha", "relay_guard_beta", "wayfinder_scout", "field_mender"]
  and .rts_evidence_bevy_runtime_adapter.creep_camp_units_sample == ["forest_alpha_creep", "forest_stalker", "forest_shaman"]
  and .rts_evidence_bevy_runtime_adapter.command_parts_samples == [["claim", "relay_beacon", "9,2"], ["clear", "forest_creep_camp", "8,3"], ["mark", "enemy_base", "10,2"], ["pressure", "counter_wave", "enemy_gate"], ["upgrade", "signal_blade", "training_hall"], ["train", "mixed_vanguard", "training_hall"], ["breach", "enemy_barracks", "10,3"], ["destroy", "enemy_barracks", "10,3"], ["level", "mirror_captain", "forest_relay"], ["claim", "forest_relay", "9,2"], ["tech", "stonebreak_cart", "relay_outpost"]]
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.kind == "control-group"
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.target_id == "5"
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.player_label == "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.kind == "move"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.tile_id == "7,4"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.player_label == "MAP MOVE SENT 7,4"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.kind == "ability"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.tile_id == "6,5"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.target_id == "arena_creep_attack"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.player_label == "COMMAND BAR ABILITY SENT FOCUS FIRE"
  and .rts_evidence_bevy_runtime_adapter.order_queue_replay_action_samples == [{"kind":"attack","payload":"arena_creep_attack"},{"kind":"move","payload":"9,2:line"},{"kind":"move","payload":"minimap:rally:5,2"},{"kind":"queue","payload":"train:worker"},{"kind":"select-control-group","payload":"3"},{"kind":"ability","payload":"focus_fire"}]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_stage_sample == "group_27_override"
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_stage_names_sample == ["group_26_queued","group_27_override","group_28_formation","group_28_filtered"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_action_payloads_sample == ["18,31:line","27","1,31:line","1,31:line"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_focus_tiles_sample == ["18,30","21,30","1,30","1,30"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_filtered_members_sample == ["missing:multi0.recall.formation.missing","foreign:map.actor1"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_stage_sample == "dimmed"
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_stage_names_sample == ["fresh","dimmed","cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_action_payloads_sample == ["18,31:line","1,31:line","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_age_ticks_sample == [0,4,8]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_events_sample == ["control_group_command_feedback_lifecycle:fresh","control_group_command_feedback_lifecycle:dimmed","control_group_command_feedback_lifecycle:cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_step_names_sample == ["select_group_26","queue_group_26","select_group_27","override_group_27","select_group_28","formation_group_28","bounded_history_after_clear"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_preview_stages_sample == ["group_26_queued","group_27_override","group_28_formation","cleared_history_bounded"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_retained_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_history_badges_sample == ["QUEUE","CANCEL_FINAL","FORMATION_FILTER_CLEAR"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_step_names_sample == ["move_without_group_selection","select_group_26_setup","move_invalid_tile_after_selection","attack_without_target","ability_before_attack_target","queue_without_queue_id","queue_unaffordable_build_after_selection","select_without_group_id"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_preview_stages_sample == ["group_selection_required","invalid_tile","attack_target_required","history_preserved_after_rejections"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_input_sources_sample == ["classic_rts_mouse_viewport","classic_rts_hotkey","classic_rts_mouse_viewport","classic_rts_mouse_viewport","classic_rts_hotkey","classic_rts_mouse_sidebar","classic_rts_mouse_sidebar","classic_rts_hotkey"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_blocked_reasons_sample == ["rts_group_selection_required","rts_invalid_tile:bad-tile","rts_attack_target_required","rts_attack_required_before_ability","rts_queue_id_required","rts_queue_unaffordable:build:watch_tower@7,4","rts_group_id_required"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_visual_stages_sample == ["group_selection_required","invalid_tile","attack_target_required","history_preserved_after_rejections"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_retained_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_history_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_stage_names_sample == ["fresh_history_appended","dimmed_history_retained","cleared_history_retained"]
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_lifecycle_stages_sample == ["fresh","dimmed","cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_stage_names_sample == ["overflow_input_pruned","recent_three_retained","cleared_history_bounded"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_prune_reasons_sample == ["recent_three_capacity","recent_three_capacity"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_feedback_kind_samples == ["move","follow","attack","harvest"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_target_label_samples == ["8,4","player","arena_creep_attack","gold_vein"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_player_label_samples == ["MOVE EXECUTING 8,4","FOLLOWING PLAYER","ATTACK FOCUS ARENA CREEP ATTACK","HARVEST GOLD VEIN TO TOWN HALL"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_target_tile_samples == [{"x":8,"y":4},{"x":5,"y":4},{"x":6,"y":5},{"x":3,"y":3}]
  and .rts_evidence_bevy_runtime_adapter.hover_target_preview_kind_sample == "attack"
  and .rts_evidence_bevy_runtime_adapter.hover_cursor_kind_sample == "ability"
  and .rts_evidence_bevy_runtime_adapter.hover_cursor_label_sample == "COMMAND BAR CURSOR ABILITY READY"
  and .rts_evidence_bevy_runtime_adapter.blocked_cursor_kind_sample == "blocked"
  and .rts_evidence_bevy_runtime_adapter.blocked_cursor_label_sample == "MAP CURSOR BLOCKED LOCK"
  and .rts_evidence_bevy_runtime_adapter.hover_player_label_sample == "MAP ATTACK READY SQUARE CREEP WANDER"
  and .rts_evidence_bevy_runtime_adapter.hover_queue_player_label_sample == "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
  and .rts_evidence_bevy_runtime_adapter.blocked_hover_player_label_sample == "MAP MOVE LOCK SELECT UNITS"
  and .rts_evidence_bevy_runtime_adapter.unit_status_stage_sample == "commander"
  and .rts_evidence_bevy_runtime_adapter.unit_status_unit_id_sample == "mirror_captain"
  and .rts_evidence_bevy_runtime_adapter.unit_status_health_sample == 76
  and .rts_evidence_bevy_runtime_adapter.unit_status_energy_sample == 68
  and .rts_evidence_bevy_runtime_adapter.unit_status_role_badges_sample == ["AUR","LVL","CMD"]
  and .rts_evidence_bevy_runtime_adapter.selection_feedback_stage_sample == "attack_lock"
  and .rts_evidence_bevy_runtime_adapter.ability_tooltip_stage_sample == "range_preview"
  and .rts_evidence_bevy_runtime_adapter.control_group_hotkey_feedback_stage_sample == "double_tap_camera"
  and .rts_bevy_runtime_map_projection.map_x == 16
  and .rts_bevy_runtime_map_projection.map_y == 54
  and .rts_bevy_runtime_map_projection.cell_w == 28
  and .rts_bevy_runtime_map_projection.cell_h == 14
  and .rts_bevy_runtime_map_projection.map_w == 952
  and .rts_bevy_runtime_map_projection.map_h == 476
  and .rts_bevy_runtime_tile_rect_sample.x == 464
  and .rts_bevy_runtime_tile_rect_sample.y == 278
  and .rts_bevy_runtime_tile_rect_sample.width == 28
  and .rts_bevy_runtime_tile_rect_sample.height == 14
  and .rts_bevy_runtime_terrain_seed_sample.surface_seed == 12
  and .rts_bevy_runtime_terrain_seed_sample.detail_seed == 20
  and .rts_bevy_runtime_map_projection_gate == true
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_x == 16
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_y == 54
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.cell_w == 28
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.cell_h == 14
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_w == 952
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection.map_h == 476
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.x == 464
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.y == 278
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.width == 28
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample.height == 14
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample.surface_seed == 12
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample.detail_seed == 20
  and .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate == true
  and .rts_bevy_runtime_map_projection == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection
  and .rts_bevy_runtime_tile_rect_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_tile_rect_sample
  and .rts_bevy_runtime_terrain_seed_sample == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_terrain_seed_sample
  and .rts_bevy_runtime_map_projection_gate == .rts_evidence_bevy_runtime_adapter.first_contact_runtime_map_projection_gate
  and .rts_online_contract == "trnm_rts_online_protocol_v1"
  and .rts_online_protocol_fixture.contract_version == "trnm_rts_online_first_contact_fixture_v1"
  and .online_protocol_fixture_field_count == (.rts_online_protocol_fixture | keys | length)
  and .online_protocol_fixture_field_count == 7
  and .rts_online_protocol_fixture.green == true
  and .rts_online_protocol_fixture.envelope.map_id == "first_contact_basin"
  and (.rts_online_protocol_fixture.envelope.update_sha256 | length) == 64
  and (.rts_online_protocol_fixture.envelope.scope.visible_chunks | length) == 3
  and (.rts_online_protocol_fixture.envelope.scope.fogged_chunks | length) == 2
  and (.rts_online_protocol_fixture.envelope.scope.visible_actor_ids | index("trnm.flux.beacon.center") != null)
  and .rts_online_protocol_fixture.authority.contract_version == "trnm_rts_online_authority_v1"
  and .rts_online_protocol_fixture.authority.green == true
  and .rts_online_protocol_fixture.authority.authority_tick == 43
  and (.rts_online_protocol_fixture.authority.authority_sha256 | length) == 64
  and (.rts_online_protocol_fixture.authority.client_requests | length) == 1
  and .online_protocol_authority_field_count == (.rts_online_protocol_fixture.authority | keys | length)
  and .online_protocol_authority_field_count == 10
  and (.rts_online_protocol_fixture.authority.accepted_orders | length) == 1
  and (.rts_online_protocol_fixture.authority.accepted_orders[0].source == "server")
  and (.rts_online_protocol_fixture.authority.rejected_orders | length) == 1
  and .rts_online_protocol_fixture.authority.rejected_orders[0].reason == "target_actor_not_visible"
  and (.rts_online_protocol_fixture.authority.scoped_updates | length) == 1
  and (.rts_online_protocol_fixture.authority.scoped_updates[0].update_sha256 | length) == 64
  and (.rts_online_protocol_fixture.authority.scoped_updates[0].scope.visible_actor_ids | index("trnm.enemy.keep.fogged") == null)
  and .rts_online_protocol_fixture.transport.contract_version == "trnm_rts_online_loopback_transport_v1"
  and .online_protocol_transport_field_count == (.rts_online_protocol_fixture.transport | keys | length)
  and .online_protocol_transport_field_count == 14
  and .rts_online_protocol_fixture.transport.green == true
  and .rts_online_protocol_fixture.transport.session_id == "first-contact-loopback-session"
  and .rts_online_protocol_fixture.transport.request_frame.direction == "client_to_server"
  and .rts_online_protocol_fixture.transport.request_frame.payload_kind == "client_request"
  and .rts_online_protocol_fixture.transport.request_frame.wire_magic == "TRNMRTS1"
  and (.rts_online_protocol_fixture.transport.request_frame.encoded_len > 96)
  and (.rts_online_protocol_fixture.transport.request_frame.payload_sha256 | length) == 64
  and (.rts_online_protocol_fixture.transport.request_frame.frame_sha256 | length) == 64
  and .rts_online_protocol_fixture.transport.response_frame.direction == "server_to_client"
  and .rts_online_protocol_fixture.transport.response_frame.payload_kind == "scoped_update"
  and .rts_online_protocol_fixture.transport.response_frame.wire_magic == "TRNMRTS1"
  and (.rts_online_protocol_fixture.transport.response_frame.encoded_len > 96)
  and (.rts_online_protocol_fixture.transport.response_frame.payload_sha256 | length) == 64
  and (.rts_online_protocol_fixture.transport.response_frame.frame_sha256 | length) == 64
  and .rts_online_protocol_fixture.transport.request_ack_matches_envelope == true
  and .rts_online_protocol_fixture.transport.response_matches_authority == true
  and .rts_online_protocol_fixture.transport.server_authoritative == true
  and .rts_online_protocol_fixture.transport.visibility_scoped_response == true
  and .rts_online_protocol_fixture.transport.socket_opened == false
  and .rts_online_protocol_fixture.transport.hosted_service_claimed == false
  and .rts_online_protocol_fixture.transport.public_launch_ready == false
  and .rts_online_protocol_fixture.lifecycle.phase == "playing"
  and .rts_online_protocol_fixture.lifecycle.bot_count == 1
  and .rts_online_protocol_gate == true
  and .rts_online_local_handoff_contract == "trnm_rts_online_local_handoff_v1"
  and .rts_online_local_handoff.contract_version == "trnm_rts_online_local_handoff_v1"
  and .online_local_handoff_field_count == (.rts_online_local_handoff | keys | length)
  and .online_local_handoff_field_count == 25
  and .rts_online_local_handoff.green == true
  and .rts_online_local_handoff.handoff_ready == true
  and .rts_online_local_handoff.handoff_id == "first-contact-local-loopback-handoff"
  and .rts_online_local_handoff.map_id == "first_contact_basin"
  and .rts_online_local_handoff.player_id == "mirror_guard"
  and .rts_online_local_handoff.phase == "playing"
  and .rts_online_local_handoff.authority_tick == 43
  and .rts_online_local_handoff.accepted_order_count == 1
  and .rts_online_local_handoff.rejected_order_count == 1
  and .rts_online_local_handoff.scoped_update_count == 1
  and .rts_online_local_handoff.bot_count == 1
  and .rts_online_local_handoff.visible_chunk_count == 3
  and .rts_online_local_handoff.visible_actor_count == 4
  and .rts_online_local_handoff.loopback_session_id == "first-contact-loopback-session"
  and (.rts_online_local_handoff.request_frame_sha256 | length) == 64
  and (.rts_online_local_handoff.response_frame_sha256 | length) == 64
  and .rts_online_local_handoff.bevy_client_role == "visualization_and_local_input_submitter"
  and .rts_online_local_handoff.authority_role == "trnm_rts_online_fixture_authority_no_socket"
  and .rts_online_local_handoff.server_authoritative == true
  and .rts_online_local_handoff.visibility_scoped_response == true
  and .rts_online_local_handoff.socket_opened == false
  and .rts_online_local_handoff.hosted_service_claimed == false
  and .rts_online_local_handoff.public_launch_ready == false
  and .rts_online_local_handoff_gate == true
  and .rts_online_offline_adapter_contract == "trnm_rts_online_offline_adapter_v1"
  and .rts_online_offline_adapter.contract_version == "trnm_rts_online_offline_adapter_v1"
  and .rts_online_offline_adapter.green == true
  and .rts_online_offline_adapter.adapter_id == "first-contact-offline-loopback-adapter"
  and .rts_online_offline_adapter.handoff_id == "first-contact-local-loopback-handoff"
  and .rts_online_offline_adapter.map_id == "first_contact_basin"
  and .rts_online_offline_adapter.adapter_mode == "offline_loopback_authority"
  and .rts_online_offline_adapter.connected_player_ids == ["local-player", "mirror_guard"]
  and .rts_online_offline_adapter.bot_player_ids == ["mirror_guard"]
  and .rts_online_offline_adapter.input_queue_labels == ["client:move_worker@8,4", "client:attack_fogged_keep"]
  and .rts_online_offline_adapter.accepted_server_order_labels == ["client:move_worker@8,4"]
  and .rts_online_offline_adapter.rejected_client_order_reasons == ["target_actor_not_visible"]
  and (.rts_online_offline_adapter.scoped_update_actor_ids | length) == 4
  and .rts_online_offline_adapter.scoped_update_order_count == 1
  and (.rts_online_offline_adapter.frame_sha256s | length) == 3
  and all(.rts_online_offline_adapter.frame_sha256s[]; length == 64)
  and .rts_online_offline_adapter_local_replay_contract == "trnm_rts_online_offline_adapter_local_replay_v1"
  and .rts_online_offline_adapter_runtime_handoff_contract == "trnm_rts_online_offline_adapter_runtime_handoff_v1"
  and .rts_online_offline_adapter.local_action_replay.contract_version == "trnm_rts_online_offline_adapter_local_replay_v1"
  and .rts_online_offline_adapter.local_action_replay.replay_mode == "bevy_local_ui_action_replay"
  and .rts_online_offline_adapter.local_action_replay.accepted_action_labels == ["RTS:SELECT:26", "RTS:MOVE:18,31:line", "RTS:SELECT:27", "RTS:MOVE:21,25:line", "RTS:SELECT:28", "RTS:MOVE:1,31:line", "RTS:SELECT:26"]
  and .rts_online_offline_adapter.local_action_replay.accepted_preview_stages == ["group_26_queued", "group_27_override", "group_28_formation", "cleared_history_bounded"]
  and .rts_online_offline_adapter.local_action_replay.blocked_action_labels == ["RTS:MOVE:18,31:line", "RTS:MOVE:bad-tile:line", "RTS:ATTACK:", "RTS:ABILITY:guard_break", "RTS:QUEUE:", "RTS:QUEUE:build:watch_tower@7,4", "RTS:SELECT:"]
  and .rts_online_offline_adapter.local_action_replay.blocked_input_sources == ["classic_rts_mouse_viewport", "classic_rts_mouse_viewport", "classic_rts_mouse_viewport", "classic_rts_hotkey", "classic_rts_mouse_sidebar", "classic_rts_mouse_sidebar", "classic_rts_hotkey"]
  and .rts_online_offline_adapter.local_action_replay.blocked_reasons == ["rts_group_selection_required", "rts_invalid_tile:bad-tile", "rts_attack_target_required", "rts_attack_required_before_ability", "rts_queue_id_required", "rts_queue_unaffordable:build:watch_tower@7,4", "rts_group_id_required"]
  and .rts_online_offline_adapter.local_action_replay.blocked_preview_stages == ["group_selection_required", "invalid_tile", "attack_target_required", "history_preserved_after_rejections"]
  and .rts_online_offline_adapter.local_action_replay.retained_history_group_ids == ["26", "27", "28"]
  and .rts_online_offline_adapter.local_action_replay.pruned_history_group_ids == ["25", "24"]
  and .rts_online_offline_adapter.local_action_replay.command_history_capacity == 3
  and .rts_online_offline_adapter.local_action_replay.local_input_sources_ready == true
  and .rts_online_offline_adapter.local_action_replay.command_history_ready == true
  and .rts_online_offline_adapter.local_action_replay.green == true
  and .rts_online_offline_adapter.local_runtime_handoff.contract_version == "trnm_rts_online_offline_adapter_runtime_handoff_v1"
  and .rts_online_offline_adapter.local_runtime_handoff.handoff_mode == "server_authoritative_runtime_command_handoff"
  and .rts_online_offline_adapter.local_runtime_handoff.accepted_runtime_command_labels == ["move:8,4"]
  and .rts_online_offline_adapter.local_runtime_handoff.accepted_runtime_destination_tile_ids == ["8,4"]
  and .rts_online_offline_adapter.local_runtime_handoff.accepted_runtime_subject_actor_ids == ["trnm.worker.alpha"]
  and .rts_online_offline_adapter.local_runtime_handoff.rejected_runtime_command_labels == ["client:attack_fogged_keep"]
  and (.rts_online_offline_adapter.local_runtime_handoff.scoped_update_actor_ids | index("trnm.worker.alpha") != null)
  and (.rts_online_offline_adapter.local_runtime_handoff.scoped_update_actor_ids | index("trnm.enemy.keep.fogged") == null)
  and .rts_online_offline_adapter.local_runtime_handoff.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
  and .rts_online_offline_adapter.local_runtime_handoff.runtime_command_stamp_kind == "server_accepted_move"
  and .rts_online_offline_adapter.local_runtime_handoff.runtime_command_stamp_tile_id == "8,4"
  and .rts_online_offline_adapter.local_runtime_handoff.accepted_order_runtime_ready == true
  and .rts_online_offline_adapter.local_runtime_handoff.rejected_order_runtime_ready == true
  and .rts_online_offline_adapter.local_runtime_handoff.scoped_update_runtime_ready == true
  and .rts_online_offline_adapter.local_runtime_handoff.no_socket_boundary_ready == true
  and .rts_online_offline_adapter.local_runtime_handoff.green == true
  and .rts_online_offline_adapter.local_multiplayer_ready == true
  and .rts_online_offline_adapter.offline_bot_ready == true
  and .rts_online_offline_adapter.bevy_adapter_ready == true
  and .rts_online_offline_adapter.server_authoritative == true
  and .rts_online_offline_adapter.visibility_scoped_response == true
  and .rts_online_offline_adapter.client_prediction_claimed == false
  and .rts_online_offline_adapter.rollback_netcode_claimed == false
  and .rts_online_offline_adapter.socket_opened == false
  and .rts_online_offline_adapter.hosted_service_claimed == false
  and .rts_online_offline_adapter.public_launch_ready == false
  and .offline_adapter_field_count == (.rts_online_offline_adapter | keys | length)
  and .offline_adapter_field_count == 29
  and .rts_online_offline_adapter_gate == true
  and .rts_online_offline_adapter_consumption.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1"
  and .offline_consumption_field_count == (.rts_online_offline_adapter_consumption | keys | length)
  and .offline_consumption_field_count == 45
  and .rts_online_offline_adapter_consumption.green == true
  and .rts_online_offline_adapter_consumption.adapter_contract == "trnm_rts_online_offline_adapter_v1"
  and .rts_online_offline_adapter_consumption.adapter_runtime_handoff_contract == "trnm_rts_online_offline_adapter_runtime_handoff_v1"
  and .rts_online_offline_adapter_consumption.adapter_runtime_handoff.green == true
  and .rts_online_offline_adapter_consumption.runtime_application_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1"
  and .rts_online_offline_adapter_consumption.runtime_application.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1"
  and .rts_online_offline_adapter_consumption.runtime_application.green == true
  and .rts_online_offline_adapter_consumption.runtime_application.handoff_contract == "trnm_rts_online_offline_adapter_runtime_handoff_v1"
  and .rts_online_offline_adapter_consumption.runtime_application.command_queue == ["move:8,4"]
  and .rts_online_offline_adapter_consumption.runtime_application.selected_unit_ids == ["trnm.worker.alpha"]
  and .rts_online_offline_adapter_consumption.runtime_application.group_route_tile_ids == ["8,4"]
  and .rts_online_offline_adapter_consumption.runtime_application.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
  and .rts_online_offline_adapter_consumption.runtime_application.runtime_command_stamp_kind == "server_accepted_move"
  and .rts_online_offline_adapter_consumption.runtime_application.runtime_command_stamp_tile_id == "8,4"
  and .rts_online_offline_adapter_consumption.runtime_application.accepted_order_runtime_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application.rejected_order_runtime_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application.scoped_update_runtime_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application.no_socket_boundary_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application.runtime_application_path == "trnm-rts-bevy-runtime offline_adapter_runtime_application -> NativeFirstPlayableRuntime mutation"
  and .rts_online_offline_adapter_consumption.adapter_mode == "offline_loopback_authority"
  and .rts_online_offline_adapter_consumption.input_queue_labels == ["client:move_worker@8,4", "client:attack_fogged_keep"]
  and .rts_online_offline_adapter_consumption.accepted_server_order_labels == ["client:move_worker@8,4"]
  and .rts_online_offline_adapter_consumption.accepted_runtime_command_labels == ["move:8,4"]
  and .rts_online_offline_adapter_consumption.accepted_runtime_destination_tile_ids == ["8,4"]
  and .rts_online_offline_adapter_consumption.accepted_runtime_subject_actor_ids == ["trnm.worker.alpha"]
  and .rts_online_offline_adapter_consumption.rejected_client_order_reasons == ["target_actor_not_visible"]
  and .rts_online_offline_adapter_consumption.rejected_runtime_command_labels == ["client:attack_fogged_keep"]
  and .rts_online_offline_adapter_consumption.rejected_commands_suppressed == true
  and (.rts_online_offline_adapter_consumption.scoped_update_actor_ids | index("trnm.worker.alpha") != null)
  and (.rts_online_offline_adapter_consumption.scoped_update_actor_ids | index("trnm.enemy.keep.fogged") == null)
  and .rts_online_offline_adapter_consumption.runtime_control_group_id == "1"
  and .rts_online_offline_adapter_consumption.runtime_group_command_state == "offline_adapter_authority_applied"
  and .rts_online_offline_adapter_consumption.runtime_pathing_status == "offline_adapter_replay_consumed"
  and .rts_online_offline_adapter_consumption.runtime_unit_response_state == "server_authoritative_move_applied"
  and .rts_online_offline_adapter_consumption.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
  and .rts_online_offline_adapter_consumption.runtime_command_stamp_kind == "server_accepted_move"
  and .rts_online_offline_adapter_consumption.runtime_command_stamp_tile_id == "8,4"
  and (.rts_online_offline_adapter_consumption.runtime_last_feedback | contains("rejected target_actor_not_visible"))
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.map_scene == "first_contact_basin"
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.current_room_id == "first-contact-basin"
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.coins == 890
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.xp == 92
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.camera_focus_tile_id == "16,16"
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.visibility_percent == 76
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.army_supply_used == 12
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.army_supply_cap == 22
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.objective_status == "secure first relay beacon and hold the center lane"
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.production_queue == ["train:guard", "train:worker", "upgrade:signal_blade"]
  and .offline_consumption_production_queue_count == (.rts_online_offline_adapter_consumption.runtime_player_screen_review.production_queue | length)
  and .offline_consumption_production_queue_count == 3
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.build_queue == ["build:watch_tower", "upgrade:training_hall"]
  and .offline_consumption_build_queue_count == (.rts_online_offline_adapter_consumption.runtime_player_screen_review.build_queue | length)
  and .offline_consumption_build_queue_count == 2
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.selected_unit_ids == ["trnm.worker.alpha"]
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.command_queue == ["move:8,4"]
  and .offline_consumption_command_queue_count == (.rts_online_offline_adapter_consumption.runtime_player_screen_review.command_queue | length)
  and .offline_consumption_command_queue_count == 1
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.command_destination_tile_id == "8,4"
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.group_route_tile_ids == ["8,4"]
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.visible_tile_count == 64
  and .offline_consumption_visible_tile_count == ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.visible_tile_ids // []) | length)
  and .offline_consumption_visible_tile_count == 0
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.fogged_tile_count == 6
  and .offline_consumption_fogged_tile_count == ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.fogged_tile_ids // []) | length)
  and .offline_consumption_fogged_tile_count == 0
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.selection_box_tile_count == 4
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.unit_health_percents == [96,78,71,34]
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.ability_command_ids == ["worker", "scout", "warden", "relay", "core", "signal"]
  and .offline_consumption_ability_command_count == (.rts_online_offline_adapter_consumption.runtime_player_screen_review.ability_command_ids | length)
  and .offline_consumption_ability_command_count == 6
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.ability_cooldown_percents == [0,0,16,0,42,25]
  and .rts_online_offline_adapter_consumption.runtime_player_screen_review.active_ability_id == "worker"
  and .rts_online_offline_adapter_consumption.local_session_handoff_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application_gate == true
  and .rts_online_offline_adapter_consumption.player_screen_review_gate == true
  and .rts_online_offline_adapter_consumption.accepted_order_runtime_gate == true
  and .rts_online_offline_adapter_consumption.rejected_order_runtime_gate == true
  and .rts_online_offline_adapter_consumption.scoped_update_runtime_gate == true
  and .rts_online_offline_adapter_consumption.no_network_claim_gate == true
  and .rts_online_offline_adapter_consumption.server_authoritative == true
  and .rts_online_offline_adapter_consumption.visibility_scoped_response == true
  and .rts_online_offline_adapter_consumption.client_prediction_claimed == false
  and .rts_online_offline_adapter_consumption.rollback_netcode_claimed == false
  and .rts_online_offline_adapter_consumption.socket_opened == false
  and .rts_online_offline_adapter_consumption.hosted_service_claimed == false
  and .rts_online_offline_adapter_consumption.public_launch_ready == false
  and .rts_online_offline_adapter_consumption.input_path == "trnm-rts-online offline adapter review input -> trnm-rts-bevy-runtime runtime application -> Bevy local player-screen snapshot"
  and .rts_online_offline_adapter_consumption.runtime_path == "trnm-rts-bevy-runtime offline_adapter_runtime_application + first_contact_offline_adapter_consumption_review -> NativeFirstPlayableRuntime consumer"
  and (.rts_online_offline_adapter_consumption.source_of_truth | contains("Bevy-free runtime application"))
  and (.rts_online_offline_adapter_consumption.source_of_truth | contains("trnm-rts-online-owned review input"))
  and (.rts_online_offline_adapter_consumption.source_of_truth | contains("player-screen/session surface"))
  and .rts_online_offline_adapter_consumption_gate == true
  and .rts_online_offline_adapter_session_transition_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1"
  and .rts_online_offline_adapter_session_transition.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1"
  and .offline_session_transition_field_count == (.rts_online_offline_adapter_session_transition | keys | length)
  and .offline_session_transition_field_count == 32
  and .rts_online_offline_adapter_session_transition.green == true
  and .rts_online_offline_adapter_session_transition.initial_application_contract == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_online_offline_adapter_session_transition.runtime_application_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1"
  and .rts_online_offline_adapter_session_transition.handoff_contract == "trnm_rts_online_offline_adapter_runtime_handoff_v1"
  and .rts_online_offline_adapter_session_transition.map_scene == "first_contact_basin"
  and .rts_online_offline_adapter_session_transition.current_room_id == "first-contact-basin"
  and .rts_online_offline_adapter_session_transition.camera_focus_tile_id == "16,16"
  and (.rts_online_offline_adapter_session_transition.before_command_queue | index("build:trnm.flux.relay") != null)
  and .rts_online_offline_adapter_session_transition.after_command_queue == ["move:8,4"]
  and (.rts_online_offline_adapter_session_transition.before_route_tile_ids | index("16,9") != null)
  and .rts_online_offline_adapter_session_transition.after_route_tile_ids == ["8,4"]
  and .rts_online_offline_adapter_session_transition.before_command_destination_tile_id == "16,9"
  and .rts_online_offline_adapter_session_transition.after_command_destination_tile_id == "8,4"
  and .rts_online_offline_adapter_session_transition.selected_unit_ids == ["trnm.worker.alpha"]
  and (.rts_online_offline_adapter_session_transition.scoped_update_actor_ids | index("trnm.worker.alpha") != null)
  and (.rts_online_offline_adapter_session_transition.scoped_update_actor_ids | index("trnm.enemy.keep.fogged") == null)
  and .rts_online_offline_adapter_session_transition.accepted_runtime_command_labels == ["move:8,4"]
  and .rts_online_offline_adapter_session_transition.rejected_runtime_command_labels == ["client:attack_fogged_keep"]
  and .rts_online_offline_adapter_session_transition.runtime_control_group_id == "1"
  and .rts_online_offline_adapter_session_transition.runtime_group_command_state == "offline_adapter_authority_applied"
  and .rts_online_offline_adapter_session_transition.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
  and .rts_online_offline_adapter_session_transition.runtime_command_stamp_kind == "server_accepted_move"
  and .rts_online_offline_adapter_session_transition.runtime_command_stamp_tile_id == "8,4"
  and (.rts_online_offline_adapter_session_transition.runtime_last_feedback | contains("rejected target_actor_not_visible"))
  and .rts_online_offline_adapter_session_transition.command_surface_replaced_gate == true
  and .rts_online_offline_adapter_session_transition.route_overlay_replaced_gate == true
  and .rts_online_offline_adapter_session_transition.session_context_preserved_gate == true
  and .rts_online_offline_adapter_session_transition.rejected_order_suppressed_gate == true
  and .rts_online_offline_adapter_session_transition.no_socket_boundary_gate == true
  and .rts_online_offline_adapter_session_transition.input_path == "trnm-rts-data player-screen application + trnm-rts-online offline adapter handoff -> trnm-rts-bevy-runtime session transition review"
  and .rts_online_offline_adapter_session_transition.runtime_path == "trnm-rts-bevy-runtime first_contact_offline_adapter_session_transition -> Bevy local session UI transition evidence"
  and (.rts_online_offline_adapter_session_transition.source_of_truth | contains("server-authoritative offline adapter handoff"))
  and .rts_online_offline_adapter_session_transition_gate == true
  and .rts_online_offline_adapter_lobby_ready_contract == "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
  and .rts_online_offline_adapter_lobby_ready.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
  and .offline_lobby_ready_field_count == (.rts_online_offline_adapter_lobby_ready | keys | length)
  and .offline_lobby_ready_field_count == 23
  and .rts_online_offline_adapter_lobby_ready.green == true
  and .rts_online_offline_adapter_lobby_ready.adapter_contract == "trnm_rts_online_offline_adapter_v1"
  and .rts_online_offline_adapter_lobby_ready.adapter_id == "first-contact-offline-loopback-adapter"
  and .rts_online_offline_adapter_lobby_ready.handoff_id == "first-contact-local-loopback-handoff"
  and .rts_online_offline_adapter_lobby_ready.arena_id == "first-contact-local-arena"
  and .rts_online_offline_adapter_lobby_ready.map_id == "first_contact_basin"
  and .rts_online_offline_adapter_lobby_ready.adapter_mode == "offline_loopback_authority"
  and .rts_online_offline_adapter_lobby_ready.bevy_client_role == "visualization_and_local_input_submitter"
  and .rts_online_offline_adapter_lobby_ready.authority_role == "trnm_rts_online_fixture_authority_no_socket"
  and .rts_online_offline_adapter_lobby_ready.connected_player_ids == ["local-player", "mirror_guard"]
  and .rts_online_offline_adapter_lobby_ready.bot_player_ids == ["mirror_guard"]
  and (.rts_online_offline_adapter_lobby_ready.ready_state_labels | index("player:local-player:ready") != null)
  and (.rts_online_offline_adapter_lobby_ready.ready_state_labels | index("player:mirror_guard:ready") != null)
  and (.rts_online_offline_adapter_lobby_ready.ready_state_labels | index("bot:mirror_guard:ready") != null)
  and (.rts_online_offline_adapter_lobby_ready.ready_state_labels | index("authority:offline_loopback:no_socket") != null)
  and .offline_lobby_ready_label_count == (.rts_online_offline_adapter_lobby_ready.ready_state_labels | length)
  and .offline_lobby_ready_label_count == 4
  and .rts_online_offline_adapter_lobby_ready.blocked_network_claim_labels == ["client_prediction:not_claimed", "rollback_netcode:not_claimed", "socket:not_claimed", "hosted_service:not_claimed", "public_launch:not_claimed"]
  and .rts_online_offline_adapter_lobby_ready.local_multiplayer_ready_gate == true
  and .rts_online_offline_adapter_lobby_ready.offline_bot_ready_gate == true
  and .rts_online_offline_adapter_lobby_ready.bevy_adapter_ready_gate == true
  and .rts_online_offline_adapter_lobby_ready.authority_ready_gate == true
  and .rts_online_offline_adapter_lobby_ready.frame_identity_gate == true
  and .rts_online_offline_adapter_lobby_ready.no_network_claim_gate == true
  and .rts_online_offline_adapter_lobby_ready.input_path == "trnm-rts-online offline adapter lobby ready input -> trnm-rts-bevy-runtime lobby ready review"
  and .rts_online_offline_adapter_lobby_ready.runtime_path == "trnm-rts-bevy-runtime first_contact_offline_adapter_lobby_ready -> Bevy local lobby/ready-state evidence"
  and (.rts_online_offline_adapter_lobby_ready.source_of_truth | contains("lobby ready review"))
  and .rts_online_offline_adapter_lobby_ready_gate == true
  and .bevy_data_actor_parity_gate == true
  and .bevy_map_model_adapter_gate == true
  and .ui_runtime_gate == true
  and (.rules[] | select(.id == "trnm.worker" and .cost == 200 and .hp == 8000))
  and (.rules[] | select(.id == "trnm.horizon.scout" and .speed == 92))
  and (.rules[] | select(.id == "trnm.forge.warden" and .hp == 18000))
  and (.rules[] | select(.id == "trnm.command.core" and .cost == 1600))
  and (.rules[] | select(.id == "trnm.flux.relay" and .cost == 500))
JQ

jq -e -f "$JQ_FILTER" "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_BASIN_SPEC_GREEN %s\n' "$OUT"
