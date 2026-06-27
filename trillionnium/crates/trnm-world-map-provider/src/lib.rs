//! Map-provider boundary for Trillionnium World.

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use trnm_world_domain::WorldState;

pub const WORLD_MAP_PROVIDER_CONTRACT: &str = "trillionnium_world_map_provider_v1";
pub const WORLD_MAP_PACK_GATE_CONTRACT: &str = "trillionnium_world_map_pack_gate_v1";
pub const WORLD_MAP_PACK_MANIFEST_CONTRACT: &str = "trillionnium_world_map_pack_manifest_v1";
pub const WORLD_MAP_PACK_ATTRIBUTION_EVIDENCE_CONTRACT: &str =
    "trillionnium_world_map_pack_attribution_evidence_v1";
pub const WORLD_MAP_PACK_SENSITIVE_POI_FILTER_CONTRACT: &str =
    "trillionnium_world_map_pack_sensitive_poi_filter_v1";
pub const OPENSTREETMAP_GEODATA_CONTRACT_VERSION: &str = "openstreetmap_geodata_v1";
pub const OPENSTREETMAP_FIXTURE_LAYERS_CONTRACT_VERSION: &str = "openstreetmap_fixture_layers_v1";
pub const OPENSTREETMAP_DERIVED_DATABASE_METADATA_CONTRACT_VERSION: &str =
    "openstreetmap_derived_database_metadata_v1";
pub const OPENSTREETMAP_PROVIDER_MODE_CONTRACT_VERSION: &str = "openstreetmap_provider_mode_v1";
pub const OPENSTREETMAP_PROVIDER_READINESS_CONTRACT_VERSION: &str =
    "openstreetmap_provider_readiness_v1";
pub const OPENSTREETMAP_GEODATA_FRESHNESS_CONTRACT_VERSION: &str =
    "openstreetmap_geodata_freshness_v1";
pub const OPENSTREETMAP_ATTRIBUTION_PRESENCE_CONTRACT_VERSION: &str =
    "openstreetmap_attribution_presence_v1";
pub const TRILLIONNIUM_WORLD_MAP_RUNTIME_PERFORMANCE_BUDGET_CONTRACT_VERSION: &str =
    "trillionnium_world_map_runtime_performance_budget_v1";
pub const TRILLIONNIUM_WORLD_MAP_RUM_SLO_CONTRACT_VERSION: &str =
    "trillionnium_world_map_rum_slo_v1";
pub const TRILLIONNIUM_WORLD_MAP_REAL_USER_RUM_MATRIX_CONTRACT_VERSION: &str =
    "trillionnium_world_map_real_user_rum_matrix_v1";
pub const TRILLIONNIUM_WORLD_MAP_DENSITY_SCALABILITY_CONTRACT_VERSION: &str =
    "trillionnium_world_map_density_scalability_v1";
pub const TRILLIONNIUM_WORLD_MAP_GAME_LAYER_SEMANTICS_CONTRACT_VERSION: &str =
    "trillionnium_world_map_game_layer_semantics_v1";
pub const TRILLIONNIUM_WORLD_MAP_GAMEPLAY_ACCESSIBILITY_CONTRACT_VERSION: &str =
    "trillionnium_world_map_gameplay_accessibility_i18n_v1";
pub const TRILLIONNIUM_WORLD_MAP_LOCATION_PRIVACY_CONTRACT_VERSION: &str =
    "trillionnium_world_map_location_privacy_v1";
pub const TRILLIONNIUM_WORLD_MAP_TRANSPORT_DELTA_CONTRACT_VERSION: &str =
    "trillionnium_world_map_transport_delta_v1";
pub const TRILLIONNIUM_WORLD_MAP_MODELING_GATE_CONTRACT_VERSION: &str =
    "trillionnium_world_map_modeling_gate_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapProviderStatus {
    pub contract_version: String,
    pub active_provider: String,
    pub geodata_contract_version: String,
    pub provider_readiness_contract_version: String,
    pub attribution_contract_version: String,
    pub fixture_only: bool,
    pub live_ingestion_enabled: bool,
    pub attribution_required: bool,
    pub map_pack_required_before_public_test: bool,
    pub cex_default_map_available: bool,
    pub cex_default_map_node_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenStreetMapProviderMode {
    Fixture,
    OverpassBboxCache,
    GeofabrikExtractImport,
    VendorTileCache,
    Unknown,
}

impl OpenStreetMapProviderMode {
    pub fn parse(value: &str) -> Self {
        match value {
            "fixture" => Self::Fixture,
            "overpass_bbox_cache" => Self::OverpassBboxCache,
            "geofabrik_extract_import" => Self::GeofabrikExtractImport,
            "vendor_tile_cache" => Self::VendorTileCache,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixture => "fixture",
            Self::OverpassBboxCache => "overpass_bbox_cache",
            Self::GeofabrikExtractImport => "geofabrik_extract_import",
            Self::VendorTileCache => "vendor_tile_cache",
            Self::Unknown => "unknown",
        }
    }

    pub fn enabled(self) -> bool {
        matches!(self, Self::Fixture)
    }

    pub fn fail_closed_reason(self) -> &'static str {
        match self {
            Self::Fixture => "fixture_provider_allowed_without_network_ingestion",
            Self::OverpassBboxCache => {
                "blocked_until_bbox_cache_rate_limit_and_odbl_tracking_exist"
            }
            Self::GeofabrikExtractImport => {
                "blocked_until_extract_import_pipeline_and_derived_database_manifest_exist"
            }
            Self::VendorTileCache => {
                "blocked_until_vendor_contract_cache_and_attribution_manifest_exist"
            }
            Self::Unknown => "unknown_provider_mode_fail_closed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenStreetMapProviderModeStatus {
    pub contract_version: String,
    pub mode: String,
    pub requested_mode: String,
    pub enabled: bool,
    pub fail_closed: bool,
    pub network_ingestion_enabled: bool,
    pub reason: String,
    pub source_of_truth: String,
}

pub fn openstreetmap_provider_mode_status(mode: &str) -> OpenStreetMapProviderModeStatus {
    let parsed = OpenStreetMapProviderMode::parse(mode);
    OpenStreetMapProviderModeStatus {
        contract_version: OPENSTREETMAP_PROVIDER_MODE_CONTRACT_VERSION.to_string(),
        mode: parsed.as_str().to_string(),
        requested_mode: mode.to_string(),
        enabled: parsed.enabled(),
        fail_closed: !parsed.enabled(),
        network_ingestion_enabled: false,
        reason: parsed.fail_closed_reason().to_string(),
        source_of_truth: "rust_openstreetmap_data_provider".to_string(),
    }
}

pub fn openstreetmap_provider_modes() -> Vec<OpenStreetMapProviderModeStatus> {
    [
        "fixture",
        "overpass_bbox_cache",
        "geofabrik_extract_import",
        "vendor_tile_cache",
        "unknown",
    ]
    .into_iter()
    .map(openstreetmap_provider_mode_status)
    .collect()
}

pub fn fixture_provider_status() -> MapProviderStatus {
    MapProviderStatus {
        contract_version: WORLD_MAP_PROVIDER_CONTRACT.to_string(),
        active_provider: "fixture_openstreetmap_v1".to_string(),
        geodata_contract_version: OPENSTREETMAP_GEODATA_CONTRACT_VERSION.to_string(),
        provider_readiness_contract_version: OPENSTREETMAP_PROVIDER_READINESS_CONTRACT_VERSION
            .to_string(),
        attribution_contract_version: OPENSTREETMAP_ATTRIBUTION_PRESENCE_CONTRACT_VERSION
            .to_string(),
        fixture_only: true,
        live_ingestion_enabled: false,
        attribution_required: true,
        map_pack_required_before_public_test: true,
        cex_default_map_available: true,
        cex_default_map_node_count: WorldState::cex_default_map_fixture().nodes.len(),
    }
}

pub fn fixture_world_from_map_provider() -> WorldState {
    WorldState::fixture()
}

pub fn cex_default_world_from_map_provider() -> WorldState {
    WorldState::cex_default_map_fixture()
}

pub fn trillionnium_world_fixture_map_pack_unsigned_manifest_json() -> Value {
    let world = cex_default_world_from_map_provider();
    let provider = fixture_provider_status();
    let canonical_payload = json!({
        "provider": provider,
        "nodes": &world.nodes,
        "edges": &world.edges,
        "positions": &world.positions,
        "source": world.source,
    });
    let payload_sha256 = world_map_sha256_json(&canonical_payload);
    json!({
        "contract_version": WORLD_MAP_PACK_MANIFEST_CONTRACT,
        "gate_contract_version": WORLD_MAP_PACK_GATE_CONTRACT,
        "map_pack_id": "trillionnium-world-fixture-osm-cex-default-v1",
        "status": "fixture_map_pack_ready_for_signature",
        "source_of_truth": "trnm_world_map_provider_cex_default_fixture",
        "provider_mode": "fixture",
        "fixture_only": true,
        "public_network_ready": false,
        "live_ingestion_enabled": false,
        "node_count": world.nodes.len(),
        "edge_count": world.edges.len(),
        "canonical_payload_sha256": payload_sha256,
        "canonical_payload": canonical_payload,
        "license": {
            "geodata_contract_version": OPENSTREETMAP_GEODATA_CONTRACT_VERSION,
            "attribution_required": true,
            "attribution_text": "© OpenStreetMap contributors",
            "database_license": "ODbL-1.0",
            "derived_database_tracking_required": true
        },
        "safety": {
            "network_ingestion_enabled": false,
            "sensitive_poi_filter_required": true,
            "geofence_policy_required_before_public_network": true,
            "takedown_runbook_required_before_public_network": true
        },
        "signature": {
            "required": true,
            "algorithm": "Ed25519",
            "status": "unsigned_pending_external_signature"
        }
    })
}

pub fn trillionnium_world_map_pack_attribution_evidence_json() -> Value {
    json!({
        "contract_version": WORLD_MAP_PACK_ATTRIBUTION_EVIDENCE_CONTRACT,
        "status": "fixture_attribution_evidence_green",
        "source_of_truth": "trnm_world_map_provider_fixture_attribution",
        "provider_mode": "fixture",
        "required_visible_text": "© OpenStreetMap contributors",
        "required_license": "ODbL-1.0",
        "surfaces": [
            {"surface": "world", "required": true, "evidence": "server_projection_contract"},
            {"surface": "app", "required": true, "evidence": "server_projection_contract"},
            {"surface": "native_bevy", "required": true, "evidence": "pending_real_device_screenshot"}
        ],
        "public_network_ready": false,
        "blocking_reason": "native_screenshot_and_public_tile_policy_still_required_before_public_network"
    })
}

pub fn trillionnium_world_map_pack_sensitive_poi_filter_report_json() -> Value {
    let world = cex_default_world_from_map_provider();
    let sensitive_terms = [
        "hospital",
        "school",
        "military",
        "embassy",
        "police",
        "religion",
        "private_home",
    ];
    let flagged_nodes = world
        .nodes
        .iter()
        .filter(|node| {
            let haystack = format!(
                "{} {} {} {}",
                node.id, node.name, node.node_kind, node.description
            )
            .to_ascii_lowercase();
            sensitive_terms.iter().any(|term| haystack.contains(term))
        })
        .map(|node| {
            json!({
                "node_id": node.id,
                "name": node.name,
                "node_kind": node.node_kind,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "contract_version": WORLD_MAP_PACK_SENSITIVE_POI_FILTER_CONTRACT,
        "status": if flagged_nodes.is_empty() { "fixture_sensitive_poi_filter_green" } else { "fixture_sensitive_poi_review_required" },
        "source_of_truth": "trnm_world_map_provider_fixture_sensitive_poi_filter",
        "provider_mode": "fixture",
        "node_count": world.nodes.len(),
        "sensitive_terms": sensitive_terms,
        "flagged_node_count": flagged_nodes.len(),
        "flagged_nodes": flagged_nodes,
        "live_ingestion_enabled": false,
        "public_network_ready": flagged_nodes.is_empty(),
        "public_network_note": "fixture scan only; live/provider map packs still require jurisdiction policy, geofence, and takedown review"
    })
}

pub fn trillionnium_world_map_modeling_gate_json() -> Value {
    let world = cex_default_world_from_map_provider();

    let building_models = world
        .nodes
        .iter()
        .map(|node| {
            let height_m = if node.node_kind.contains("tower") {
                24
            } else if node.node_kind.contains("gate")
                || node.node_kind.contains("arcade")
                || node.node_kind.contains("vault")
            {
                16
            } else if node.node_kind.contains("yard")
                || node.node_kind.contains("camp")
                || node.node_kind.contains("cistern")
            {
                4
            } else {
                10
            };
            json!({
                "model_id": format!("building:{}", node.id),
                "source_node_id": node.id,
                "display_name": node.name,
                "node_kind": node.node_kind,
                "asset_class": "building_mass_from_map_pack_node",
                "footprint": {
                    "contract": "fixture_grid_footprint_from_world_node_lat_lng_e7",
                    "center": {"x": node.lng_e7, "y": node.lat_e7},
                    "half_extent_tiles": if node.node_kind.contains("square") || node.node_kind.contains("yard") { 2 } else { 1 },
                },
                "height_m": height_m,
                "roof_profile": if node.node_kind.contains("tower") { "watch_tower_roof" } else if node.node_kind.contains("gate") { "gatehouse_roof" } else { "low_poly_city_roof" },
                "collision_role": "walkable_boundary_and_occlusion_hint",
                "gameplay_anchor_tags": node.tags,
            })
        })
        .collect::<Vec<_>>();

    let road_models = world
        .edges
        .iter()
        .enumerate()
        .map(|(idx, edge)| {
            let from_node = world.node(&edge.from);
            let to_node = world.node(&edge.to);
            json!({
                "model_id": format!("road:{:03}:{}:{}", idx + 1, edge.from, edge.to),
                "source_edge": {"from": edge.from, "to": edge.to, "direction": edge.direction},
                "asset_class": "road_path_from_map_pack_edge",
                "road_class": if edge.direction == "east" || edge.direction == "west" { "street_lane" } else { "path_lane" },
                "navigation_role": "walkable_route_graph",
                "path_polyline": [
                    {"x": from_node.map(|node| node.lng_e7).unwrap_or_default(), "y": from_node.map(|node| node.lat_e7).unwrap_or_default()},
                    {"x": to_node.map(|node| node.lng_e7).unwrap_or_default(), "y": to_node.map(|node| node.lat_e7).unwrap_or_default()}
                ],
                "material_hint": if edge.direction == "north" || edge.direction == "south" { "stone_path" } else { "packed_earth_street" },
            })
        })
        .collect::<Vec<_>>();

    let greenery_terms = [
        "yard", "garden", "camp", "water", "river", "field", "cloister", "terrain", "survey",
        "green", "forest", "park",
    ];
    let greenery_models = world
        .nodes
        .iter()
        .filter(|node| {
            let haystack = format!("{} {} {}", node.id, node.node_kind, node.tags.join(" "))
                .to_ascii_lowercase();
            greenery_terms
                .iter()
                .any(|term| haystack.contains(term))
        })
        .map(|node| {
            json!({
                "model_id": format!("greenery:{}", node.id),
                "source_node_id": node.id,
                "asset_class": "greenery_cluster_from_map_pack_tags",
                "center": {"x": node.lng_e7, "y": node.lat_e7},
                "foliage_role": if node.tags.iter().any(|tag| tag == "water") { "riverbank_reeds" } else if node.node_kind.contains("yard") { "courtyard_tree_cluster" } else { "low_shrub_and_ground_cover" },
                "density": if node.node_kind.contains("field") || node.node_kind.contains("cloister") { "medium" } else { "low" },
                "gameplay_readability": "soft_cover_and_biome_hint_not_path_authority",
                "source_tags": node.tags,
            })
        })
        .collect::<Vec<_>>();

    let terrain_models = vec![
        json!({
            "zone_id": "terrain:mirror-city-paved-lowland",
            "terrain_kind": "paved_urban_plaza",
            "source_node_ids": ["mirror-city-square", "zbj-market-gate", "client-board", "delivery-dock"],
            "elevation_band": "lowland",
            "mesh_role": "ground_surface",
            "walkability": "high",
        }),
        json!({
            "zone_id": "terrain:craft-district-yard",
            "terrain_kind": "workshop_courtyard",
            "source_node_ids": ["starter-studio", "forge-workbench", "asset-yard"],
            "elevation_band": "lowland",
            "mesh_role": "ground_surface_plus_yard_breakup",
            "walkability": "high",
        }),
        json!({
            "zone_id": "terrain:river-cistern-wetland",
            "terrain_kind": "water_edge_buffer",
            "source_node_ids": ["river-cistern", "ration-kitchen", "field-infirmary"],
            "elevation_band": "lowland_water",
            "mesh_role": "water_and_bank_surface",
            "walkability": "partial",
        }),
        json!({
            "zone_id": "terrain:survey-tower-ridge",
            "terrain_kind": "survey_ridge_blocked_path",
            "source_node_ids": ["survey-tower", "elder-step", "mentor-cloister", "auction-arcade"],
            "elevation_band": "raised_ridge",
            "mesh_role": "height_hint_and_blocked_path_surface",
            "walkability": "gated",
        }),
    ];

    let building_count = building_models.len();
    let road_count = road_models.len();
    let greenery_count = greenery_models.len();
    let terrain_count = terrain_models.len();
    let building_gate = building_count >= 20;
    let road_gate = road_count >= 20;
    let greenery_gate = greenery_count >= 5;
    let terrain_gate = terrain_count >= 4;
    let source_gate = true;
    let green = building_gate && road_gate && greenery_gate && terrain_gate && source_gate;
    let required_next_evidence = [
        "approved_production_map_source",
        "signed_production_map_pack_manifest",
        "building_footprint_derivation_report",
        "road_graph_derivation_report",
        "greenery_landuse_derivation_report",
        "terrain_mesh_derivation_report",
        "visible_attribution_screenshots",
        "sensitive_poi_and_geofence_review",
        "operator_signoff",
    ];
    let required_next_evidence_count = required_next_evidence.len();
    let gate_count = 6;
    let passed_gate_count = [
        building_gate,
        road_gate,
        greenery_gate,
        terrain_gate,
        source_gate,
        green,
    ]
    .into_iter()
    .filter(|gate| *gate)
    .count();

    json!({
        "contract_version": TRILLIONNIUM_WORLD_MAP_MODELING_GATE_CONTRACT_VERSION,
        "status": if green { "fixture_map_modeling_gate_green_with_public_data_blockers" } else { "fixture_map_modeling_gate_blocked" },
        "source_of_truth": "trnm_world_map_provider_fixture_modeling",
        "green": green,
        "provider_mode": "fixture",
        "fixture_only": true,
        "public_map_pack_ready": false,
        "public_launch_ready": false,
        "public_launch_credit": false,
        "live_ingestion_performed": false,
        "live_ingestion_enabled": false,
        "runtime_clients_fetch_public_osm_directly": false,
        "public_network_ready": false,
        "building_model_count": building_count,
        "road_model_count": road_count,
        "greenery_model_count": greenery_count,
        "terrain_model_count": terrain_count,
        "modeling_layer_count": 4,
        "required_next_evidence_count": required_next_evidence_count,
        "gate_count": gate_count,
        "passed_gate_count": passed_gate_count,
        "failed_gate_count": gate_count - passed_gate_count,
        "public_network_blocking_reason": "building/road/greenery/terrain modeling is proven on deterministic fixture map_pack only; production credit still requires approved real map-pack source, cache policy, attribution screenshots, sensitive POI/geofence review, and operator signoff",
        "modeling_layers": {
            "buildings": building_models,
            "roads": road_models,
            "greenery": greenery_models,
            "terrain": terrain_models,
        },
        "layer_counts": {
            "buildings": building_count,
            "roads": road_count,
            "greenery": greenery_count,
            "terrain": terrain_count,
        },
        "gates": {
            "building_modeling_gate": building_gate,
            "road_modeling_gate": road_gate,
            "greenery_modeling_gate": greenery_gate,
            "terrain_modeling_gate": terrain_gate,
            "no_live_ingestion_gate": source_gate,
            "all_layers_modeled": green,
        },
        "modeling_policy": {
            "building_source": "map_pack_nodes_to_low_poly_footprints",
            "road_source": "map_pack_edges_to_walkable_route_graph",
            "greenery_source": "map_pack_tags_to_foliage_clusters",
            "terrain_source": "authored_zone_meshes_bound_to_map_pack_node_groups",
            "renderer_authority": "native_bevy_visualization_only_world_state_remains_rust_authoritative",
            "production_data_rule": "real map modeling credit must consume signed production map_pack artifacts, not direct runtime Overpass or Geofabrik calls",
        },
        "required_next_evidence": required_next_evidence,
    })
}

pub fn trillionnium_percent_i64(numerator: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        0
    } else {
        ((numerator.max(0) as f64 / denominator.max(1) as f64) * 100.0).round() as i64
    }
}

pub fn trillionnium_bounded_percent_i64(numerator: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        0
    } else {
        let numerator = numerator.max(0).min(denominator.max(0));
        ((numerator as f64 / denominator.max(1) as f64) * 100.0).round() as i64
    }
}

pub fn trillionnium_retention_band(percent: i64) -> &'static str {
    if percent >= 35 {
        "healthy"
    } else if percent > 0 {
        "needs_attention"
    } else {
        "needs_instrumented_sample"
    }
}

pub fn trillionnium_world_map_density_scalability_json(
    marker_count: usize,
    avatar_route_runner_count: usize,
    payload_object_count: usize,
) -> Value {
    let dense_payload =
        payload_object_count > 48 || marker_count > 12 || avatar_route_runner_count > 4;
    json!({
        "contract_version": TRILLIONNIUM_WORLD_MAP_DENSITY_SCALABILITY_CONTRACT_VERSION,
        "status": if dense_payload { "adaptive_density_active" } else { "adaptive_density_ready" },
        "backend_projection": {
            "owner": "world_map_projection",
            "spatial_tile_cache_required": true,
            "entity_group_delta_cache_required": true,
            "server_timing_header_required": true,
            "query_plan_pressure_model": "tile_shard_count + marker_cluster_count + route_runner_count + live_event_count",
            "max_snapshot_objects_before_delta_required": 72,
        },
        "frontend_virtualization": {
            "virtualize_dense_cards_required": true,
            "changed_group_render_required": true,
            "defer_secondary_render_required": true,
            "low_end_device_marker_budget": 12,
            "low_end_device_avatar_runner_budget": 3,
            "device_memory_data_saver_budget_visible": true,
        },
        "adaptive_density_scheduler": {
            "inputs": ["zoom", "payload_object_count", "device_class", "save_data", "battery_saver", "rum_p95"],
            "actions": ["cluster_markers", "collapse_secondary_poi", "defer_cards", "reduce_avatar_runner_animation", "prefer_delta_noop"],
            "current_payload_object_count": payload_object_count,
            "current_marker_count": marker_count,
            "current_avatar_route_runner_count": avatar_route_runner_count,
        },
        "load_gate": {
            "viewport_p95_target_ms": 250,
            "server_projection_target_ms": 120,
            "delta_payload_target_objects": 24,
            "cache_hit_or_delta_noop_required": true,
        },
        "readiness_checks": [
            "spatial_tile_cache_required",
            "entity_group_delta_cache_required",
            "server_timing_required",
            "virtualized_cards_required",
            "adaptive_density_scheduler_visible",
            "device_memory_data_saver_budget_visible",
            "delta_payload_pressure_model_visible"
        ]
    })
}

pub fn trillionnium_world_map_runtime_performance_budget_json(
    marker_count: usize,
    max_visible_markers: usize,
    avatar_route_runner_count: usize,
    max_avatar_route_runners: usize,
    payload_object_count: usize,
) -> Value {
    let marker_utilization_percent =
        trillionnium_bounded_percent_i64(marker_count as i64, max_visible_markers.max(1) as i64);
    let avatar_runner_utilization_percent = trillionnium_bounded_percent_i64(
        avatar_route_runner_count as i64,
        max_avatar_route_runners.max(1) as i64,
    );
    let avatar_runner_headroom = max_avatar_route_runners.saturating_sub(avatar_route_runner_count);
    json!({
        "contract_version": TRILLIONNIUM_WORLD_MAP_RUNTIME_PERFORMANCE_BUDGET_CONTRACT_VERSION,
        "status": if avatar_runner_headroom == 0 { "within_budget_but_no_avatar_runner_headroom" } else { "within_budget_with_headroom" },
        "budget_targets": {
            "first_map_interactive_target_ms": 2000,
            "viewport_refresh_p95_target_ms": 250,
            "focus_to_action_rail_target_ms": 300,
            "main_thread_long_task_budget_ms": 100,
            "low_end_mobile_fps_floor": 45,
            "tile_error_rate_target_percent": 1
        },
        "current_pressure": {
            "visible_markers": marker_count,
            "max_visible_markers": max_visible_markers,
            "marker_utilization_percent": marker_utilization_percent,
            "avatar_route_runners": avatar_route_runner_count,
            "max_avatar_route_runners": max_avatar_route_runners,
            "avatar_runner_utilization_percent": avatar_runner_utilization_percent,
            "avatar_runner_headroom": avatar_runner_headroom,
            "payload_object_count": payload_object_count
        },
        "degrade_strategy": {
            "delta_viewport_updates_required": true,
            "abort_previous_viewport_request": true,
            "defer_noncritical_card_render": true,
            "changed_group_rendering_required": true,
            "cluster_markers_before_hiding": true,
            "low_end_device_avatar_runner_cap": 3,
            "collapse_non_route_layers_first": true,
            "aggregate_extra_runners_into_pulse": true,
            "render_order": ["active_route", "current_objective", "reward_checkpoint", "next_route_cta", "marker_clusters", "live_event_pulses", "secondary_poi"]
        },
        "density_scalability": trillionnium_world_map_density_scalability_json(
            marker_count,
            avatar_route_runner_count,
            payload_object_count,
        ),
        "readiness_checks": [
            "first_interactive_budget_visible",
            "viewport_refresh_budget_visible",
            "focus_to_action_budget_visible",
            "long_task_budget_visible",
            "low_end_mobile_floor_visible",
            "delta_update_requirement_visible",
            "viewport_request_abort_visible",
            "changed_group_rendering_visible",
            "marker_cluster_policy_visible",
            "backend_projection_cache_visible",
            "frontend_virtualization_visible",
            "adaptive_density_scheduler_visible",
            "avatar_runner_degrade_strategy_visible",
            "rum_slo_quantiles_required"
        ]
    })
}

pub fn trillionnium_world_map_real_user_rum_matrix_contract_json() -> Value {
    json!({
        "contract_version": TRILLIONNIUM_WORLD_MAP_REAL_USER_RUM_MATRIX_CONTRACT_VERSION,
        "status": "matrix_required_before_density_scale",
        "surfaces": ["app", "world"],
        "device_classes": ["mobile", "desktop"],
        "network_classes": ["normal", "weak_network_cached_snapshot"],
        "sample_cells": [
            {"surface": "world", "device_class": "mobile", "sample_kind": "cold_cache_interactive", "p95_target_ms": 2000},
            {"surface": "world", "device_class": "mobile", "sample_kind": "warm_delta_or_304", "p95_target_ms": 250},
            {"surface": "app", "device_class": "desktop", "sample_kind": "focus_to_action_rail", "p95_target_ms": 300}
        ],
        "owner": "rust_world_map_rum_gate"
    })
}

pub fn trillionnium_world_map_rum_slo_contract_json() -> Value {
    json!({
        "contract_version": TRILLIONNIUM_WORLD_MAP_RUM_SLO_CONTRACT_VERSION,
        "status": "hard_gate_quantiles_before_map_density",
        "required_dimensions": {
            "surfaces": ["app", "world"],
            "device_classes": ["mobile", "desktop"],
            "quantiles": ["p50", "p95", "p99"],
            "sample_kinds": ["cold_cache_interactive", "warm_delta_or_304", "weak_network_cached_snapshot"]
        },
        "sample_matrix": trillionnium_world_map_real_user_rum_matrix_contract_json(),
        "targets": {
            "first_map_interactive_p95_ms": 2000,
            "viewport_refresh_p95_ms": 250,
            "focus_to_action_rail_p95_ms": 300,
            "main_thread_long_task_p95_ms": 100,
            "tile_error_rate_percent": 1,
            "delta_snapshot_fallback_failure_rate_percent": 0
        },
        "evidence_sources": ["/world/web/map-rum", "/v1/world/map/{matrix_user_id}/rum", "/metrics", "/health.metrics.world_map_rum.slo_gate"],
        "readiness_checks": ["rum_contract_visible", "quantile_dimensions_visible", "weak_network_sample_required", "health_and_metrics_surface_required"]
    })
}

pub fn world_map_hash_json(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

pub fn world_map_sha256_json(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

pub fn world_map_entity_group_payloads(viewport: &Value) -> Vec<(&'static str, Value)> {
    vec![
        (
            "active_region",
            viewport
                .get("active_region")
                .cloned()
                .unwrap_or(Value::Null),
        ),
        (
            "stream_region_shards",
            viewport
                .get("stream_region_shards")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "visible_tile_shards",
            viewport
                .get("visible_tile_shards")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "prefetch_queue",
            viewport
                .get("prefetch_queue")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "visible_markers",
            viewport
                .get("visible_markers")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "poi_hotspots",
            viewport
                .get("poi_hotspots")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "marker_clusters",
            viewport
                .get("marker_clusters")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "player_avatars",
            viewport
                .get("player_avatars")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "avatar_task_routes",
            viewport
                .get("avatar_task_routes")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "avatar_route_runners",
            viewport
                .get("avatar_route_runners")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "live_event_stream",
            viewport
                .get("live_event_stream")
                .cloned()
                .unwrap_or_else(|| json!([])),
        ),
        (
            "route_runner_handoff",
            viewport
                .get("route_runner_handoff")
                .cloned()
                .unwrap_or(Value::Null),
        ),
        (
            "player_density",
            viewport
                .get("player_density")
                .cloned()
                .unwrap_or(Value::Null),
        ),
    ]
}

pub fn world_map_entity_group_versions(viewport: &Value) -> Map<String, Value> {
    let mut versions = Map::new();
    for (group_id, payload) in world_map_entity_group_payloads(viewport) {
        versions.insert(group_id.to_string(), json!(world_map_hash_json(&payload)));
    }
    versions
}

pub fn world_map_entity_versions_cursor(versions: &Map<String, Value>) -> String {
    let mut pairs = versions
        .iter()
        .filter_map(|(key, value)| value.as_str().map(|hash| format!("{key}:{hash}")))
        .collect::<Vec<_>>();
    pairs.sort();
    pairs.join(",")
}

pub fn world_map_cursor_with_entity_versions(
    base_cursor: &str,
    versions: &Map<String, Value>,
) -> String {
    format!(
        "{base_cursor};gv={}",
        world_map_entity_versions_cursor(versions)
    )
}

pub fn world_map_viewport_cursor(viewport: &Value) -> String {
    let active_region_id = viewport
        .get("active_region")
        .and_then(|region| region.get("region_id"))
        .and_then(Value::as_str)
        .unwrap_or("cn-shanghai-core");
    let tile_id = viewport
        .get("tile_center")
        .and_then(|tile| tile.get("tile_id"))
        .and_then(Value::as_str)
        .unwrap_or("tile:unknown");
    let zoom = viewport.get("zoom").and_then(Value::as_i64).unwrap_or(15);
    let latest_event_epoch = viewport
        .get("live_event_stream")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|event| event.get("created_at_epoch").and_then(Value::as_i64))
        .max()
        .unwrap_or(0);
    let latest_runner_epoch = viewport
        .get("avatar_route_runners")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|runner| {
            runner
                .get("lifecycle")
                .and_then(|lifecycle| lifecycle.get("updated_at_epoch"))
                .and_then(Value::as_i64)
        })
        .max()
        .unwrap_or(0);
    let base_cursor = format!(
        "region={active_region_id};tile={tile_id};z={zoom};e={latest_event_epoch};r={latest_runner_epoch};m={};a={}",
        viewport.get("marker_count").and_then(Value::as_i64).unwrap_or(0),
        viewport.get("player_avatar_count").and_then(Value::as_i64).unwrap_or(0)
    );
    world_map_cursor_with_entity_versions(&base_cursor, &world_map_entity_group_versions(viewport))
}

pub fn parse_world_map_entity_versions(cursor: Option<&str>) -> HashMap<String, String> {
    let Some(cursor) = cursor else {
        return HashMap::new();
    };
    let Some(group_versions) = cursor.split(";gv=").nth(1) else {
        return HashMap::new();
    };
    group_versions
        .split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once(':')?;
            let key = key.trim();
            let value = value.trim();
            (!key.is_empty() && !value.is_empty()).then(|| (key.to_string(), value.to_string()))
        })
        .collect()
}

pub fn world_map_entity_delta_json(
    viewport: &Value,
    cursor: Option<&str>,
    changed: bool,
) -> (Value, Vec<String>) {
    let versions = world_map_entity_group_versions(viewport);
    let previous_versions = parse_world_map_entity_versions(cursor);
    let changed_groups = if !changed {
        Vec::new()
    } else if previous_versions.is_empty() {
        versions.keys().cloned().collect::<Vec<_>>()
    } else {
        versions
            .iter()
            .filter_map(|(group_id, version)| {
                let current = version.as_str().unwrap_or_default();
                (previous_versions.get(group_id).map(String::as_str) != Some(current))
                    .then(|| group_id.clone())
            })
            .collect::<Vec<_>>()
    };
    let changed_group_set = changed_groups.iter().cloned().collect::<HashSet<_>>();
    let mut groups = Map::new();
    for (group_id, payload) in world_map_entity_group_payloads(viewport) {
        if changed_group_set.contains(group_id) {
            groups.insert(group_id.to_string(), payload);
        }
    }
    (
        json!({
            "contract_version": TRILLIONNIUM_WORLD_MAP_TRANSPORT_DELTA_CONTRACT_VERSION,
            "mode": "entity_group_versioned_delta_v1",
            "changed_groups_only": true,
            "cursor_carries_group_versions": true,
            "changed_group_count": changed_groups.len(),
            "changed_groups": changed_groups,
            "group_versions": versions,
            "groups": groups,
            "noop_can_reuse_cached_snapshot": !changed,
            "snapshot_fallback_is_failure": false,
            "etag_304_compatible": true,
        }),
        changed_groups,
    )
}

pub fn world_map_delta_payload_json(viewport: &Value, changed_group_ids: &[String]) -> Value {
    let changed_group_set = changed_group_ids.iter().cloned().collect::<HashSet<_>>();
    let mut payload = Map::new();
    for (group_id, value) in world_map_entity_group_payloads(viewport) {
        if changed_group_set.contains(group_id) {
            payload.insert(group_id.to_string(), value);
        }
    }
    Value::Object(payload)
}

pub fn world_map_weak_etag(cursor: &str) -> String {
    let safe = cursor
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    format!("W/\"trillionnium-map-{safe}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_provider_fails_closed_for_live_ingestion() {
        let status = fixture_provider_status();
        assert!(status.fixture_only);
        assert!(!status.live_ingestion_enabled);
        assert!(status.map_pack_required_before_public_test);
        assert_eq!(
            status.geodata_contract_version,
            OPENSTREETMAP_GEODATA_CONTRACT_VERSION
        );
        assert_eq!(status.cex_default_map_node_count, 24);
    }

    #[test]
    fn non_fixture_provider_modes_fail_closed() {
        let overpass = openstreetmap_provider_mode_status("overpass_bbox_cache");
        assert_eq!(
            overpass.contract_version,
            OPENSTREETMAP_PROVIDER_MODE_CONTRACT_VERSION
        );
        assert!(!overpass.enabled);
        assert!(overpass.fail_closed);
        assert!(!overpass.network_ingestion_enabled);

        let fixture = openstreetmap_provider_mode_status("fixture");
        assert!(fixture.enabled);
        assert!(!fixture.fail_closed);
    }

    #[test]
    fn provider_packages_cex_default_map_fixture() {
        let world = cex_default_world_from_map_provider();
        assert_eq!(world.nodes.len(), 24);
        assert!(world.node("mentor-cloister").is_some());
    }

    #[test]
    fn fixture_map_pack_manifest_and_compliance_evidence_are_available() {
        let manifest = trillionnium_world_fixture_map_pack_unsigned_manifest_json();
        assert_eq!(
            manifest["contract_version"],
            WORLD_MAP_PACK_MANIFEST_CONTRACT
        );
        assert_eq!(manifest["node_count"], 24);
        assert_eq!(manifest["signature"]["algorithm"], "Ed25519");
        assert_eq!(manifest["public_network_ready"], false);
        assert!(manifest["canonical_payload_sha256"]
            .as_str()
            .is_some_and(|hash| hash.len() == 64));

        let attribution = trillionnium_world_map_pack_attribution_evidence_json();
        assert_eq!(
            attribution["contract_version"],
            WORLD_MAP_PACK_ATTRIBUTION_EVIDENCE_CONTRACT
        );
        assert_eq!(attribution["required_license"], "ODbL-1.0");

        let sensitive = trillionnium_world_map_pack_sensitive_poi_filter_report_json();
        assert_eq!(
            sensitive["contract_version"],
            WORLD_MAP_PACK_SENSITIVE_POI_FILTER_CONTRACT
        );
        assert_eq!(sensitive["flagged_node_count"], 0);
    }

    #[test]
    fn fixture_map_pack_modeling_gate_covers_city_layers_without_live_ingestion() {
        let modeling = trillionnium_world_map_modeling_gate_json();
        assert_eq!(
            modeling["contract_version"],
            TRILLIONNIUM_WORLD_MAP_MODELING_GATE_CONTRACT_VERSION
        );
        assert_eq!(
            modeling["status"],
            "fixture_map_modeling_gate_green_with_public_data_blockers"
        );
        assert_eq!(modeling["green"], true);
        assert_eq!(modeling["fixture_only"], true);
        assert_eq!(modeling["public_map_pack_ready"], false);
        assert_eq!(modeling["public_launch_ready"], false);
        assert_eq!(modeling["public_launch_credit"], false);
        assert_eq!(modeling["live_ingestion_performed"], false);
        assert_eq!(modeling["live_ingestion_enabled"], false);
        assert_eq!(modeling["runtime_clients_fetch_public_osm_directly"], false);
        assert_eq!(
            modeling["building_model_count"],
            modeling["layer_counts"]["buildings"]
        );
        assert_eq!(
            modeling["road_model_count"],
            modeling["layer_counts"]["roads"]
        );
        assert_eq!(
            modeling["greenery_model_count"],
            modeling["layer_counts"]["greenery"]
        );
        assert_eq!(
            modeling["terrain_model_count"],
            modeling["layer_counts"]["terrain"]
        );
        assert_eq!(modeling["modeling_layer_count"], 4);
        assert_eq!(modeling["required_next_evidence_count"], 9);
        assert_eq!(modeling["gate_count"], 6);
        assert_eq!(modeling["passed_gate_count"], 6);
        assert_eq!(modeling["failed_gate_count"], 0);
        assert_eq!(modeling["gates"]["building_modeling_gate"], true);
        assert_eq!(modeling["gates"]["road_modeling_gate"], true);
        assert_eq!(modeling["gates"]["greenery_modeling_gate"], true);
        assert_eq!(modeling["gates"]["terrain_modeling_gate"], true);
        assert!(
            modeling["layer_counts"]["buildings"]
                .as_u64()
                .unwrap_or_default()
                >= 20
        );
        assert!(
            modeling["layer_counts"]["roads"]
                .as_u64()
                .unwrap_or_default()
                >= 20
        );
        assert!(
            modeling["layer_counts"]["greenery"]
                .as_u64()
                .unwrap_or_default()
                >= 5
        );
        assert!(
            modeling["layer_counts"]["terrain"]
                .as_u64()
                .unwrap_or_default()
                >= 4
        );
        assert_eq!(
            modeling["modeling_policy"]["production_data_rule"],
            "real map modeling credit must consume signed production map_pack artifacts, not direct runtime Overpass or Geofabrik calls"
        );
    }

    #[test]
    fn map_runtime_budget_and_rum_contracts_match_cex_gate_shape() {
        assert_eq!(trillionnium_percent_i64(2, 5), 40);
        assert_eq!(trillionnium_bounded_percent_i64(12, 10), 100);
        assert_eq!(trillionnium_retention_band(0), "needs_instrumented_sample");

        let budget = trillionnium_world_map_runtime_performance_budget_json(13, 20, 5, 5, 73);
        assert_eq!(
            budget["contract_version"],
            TRILLIONNIUM_WORLD_MAP_RUNTIME_PERFORMANCE_BUDGET_CONTRACT_VERSION
        );
        assert_eq!(
            budget["status"],
            "within_budget_but_no_avatar_runner_headroom"
        );
        assert_eq!(
            budget["density_scalability"]["status"],
            "adaptive_density_active"
        );

        let rum = trillionnium_world_map_rum_slo_contract_json();
        assert_eq!(
            rum["contract_version"],
            TRILLIONNIUM_WORLD_MAP_RUM_SLO_CONTRACT_VERSION
        );
        assert_eq!(
            rum["sample_matrix"]["contract_version"],
            TRILLIONNIUM_WORLD_MAP_REAL_USER_RUM_MATRIX_CONTRACT_VERSION
        );
    }

    #[test]
    fn map_delta_cursor_helpers_preserve_entity_group_semantics() {
        let viewport = json!({
            "active_region": {"region_id": "cn-shanghai-core"},
            "tile_center": {"tile_id": "tile:1"},
            "zoom": 15,
            "marker_count": 2,
            "player_avatar_count": 1,
            "visible_markers": [{"id": "m1"}],
            "live_event_stream": [{"created_at_epoch": 100}],
            "avatar_route_runners": [{"lifecycle": {"updated_at_epoch": 200}}]
        });
        let cursor = world_map_viewport_cursor(&viewport);
        assert!(cursor.contains("region=cn-shanghai-core"));
        assert!(cursor.contains(";gv="));
        let parsed = parse_world_map_entity_versions(Some(&cursor));
        assert!(parsed.contains_key("visible_markers"));

        let (initial_delta, changed_groups) = world_map_entity_delta_json(&viewport, None, true);
        assert_eq!(
            initial_delta["contract_version"],
            TRILLIONNIUM_WORLD_MAP_TRANSPORT_DELTA_CONTRACT_VERSION
        );
        assert!(changed_groups
            .iter()
            .any(|group| group == "visible_markers"));

        let (noop_delta, noop_groups) =
            world_map_entity_delta_json(&viewport, Some(&cursor), false);
        assert!(noop_groups.is_empty());
        assert_eq!(noop_delta["noop_can_reuse_cached_snapshot"], true);
        assert!(world_map_weak_etag(&cursor).starts_with("W/\"trillionnium-map-"));
    }
}
