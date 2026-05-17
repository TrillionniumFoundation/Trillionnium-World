//! Standalone Trillionnium World server/runtime smoke surface.

use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use trnm_world_api::{
    world_runtime_adapter_readiness, WorldActorIdentity, WorldApiCommandRequest,
    WorldApiCommandResponse, WorldApiFullSplitResponse, WorldApiHomeResponse,
    WorldApiMapRuntimeBudgetResponse, WorldApiRouteArtifactsResponse,
    WorldApiRouteCommandTargetResponse, WorldApiTacticsCommandResponse, WorldEvidenceReceipt,
    WorldEvidenceSink, WorldIdentityAdapter, WorldLedgerAdapter, WorldLedgerReceipt,
    WorldMetricReceipt, WorldMetricsSink, WorldRepository, WorldRepositoryReceipt,
    WorldRuntimeAdapterReadiness, WorldSessionDecision, WorldSessionGuard,
    WORLD_FULL_SPLIT_RESPONSE_CONTRACT, WORLD_RUNTIME_ADAPTER_CONTRACT,
};
use trnm_world_command::{
    apply_command, apply_tactics_command, WorldCommand, WorldTacticsCommandRequest,
};
use trnm_world_domain::{WorldState, WorldTrillionniumCharacter};
use trnm_world_map_provider::{
    cex_default_world_from_map_provider, fixture_provider_status,
    trillionnium_world_fixture_map_pack_unsigned_manifest_json,
    trillionnium_world_map_modeling_gate_json,
    trillionnium_world_map_pack_attribution_evidence_json,
    trillionnium_world_map_pack_sensitive_poi_filter_report_json,
    trillionnium_world_map_runtime_performance_budget_json,
};
use trnm_world_projection::{
    project_home, world_full_split_projection_json, world_route_artifacts_from_raw_preview_items,
    world_route_command_target, WorldRouteRecords,
};
use trnm_world_ui_fragments::{render_home_fragment, render_keypad_buttons_fragment};

pub const WORLD_SERVER_CONTRACT: &str = "trillionnium_world_server_v1";
pub const WORLD_DEV_RUNTIME_CONTRACT: &str = "trillionnium_world_dev_runtime_v1";
pub const WORLD_DEV_REPOSITORY_CONTRACT: &str = "trillionnium_world_dev_file_repository_v1";
pub const WORLD_BROWSER_PARITY_SHELL_CONTRACT: &str =
    "trillionnium_world_standalone_browser_parity_shell_v1";

#[derive(Debug, Default, Clone, Copy)]
pub struct FixtureWorldRuntimeAdapters;

impl WorldIdentityAdapter for FixtureWorldRuntimeAdapters {
    fn resolve_actor(&self, actor_id: &str) -> WorldActorIdentity {
        WorldActorIdentity {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            actor_id: actor_id.to_string(),
            matrix_user_id: actor_id.to_string(),
            display_name: "Local Trillionnium Player".to_string(),
            source_of_truth: "fixture_world_identity_adapter".to_string(),
        }
    }
}

impl WorldSessionGuard for FixtureWorldRuntimeAdapters {
    fn authorize_world_session(&self, actor_id: &str) -> WorldSessionDecision {
        WorldSessionDecision {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            accepted: true,
            session_id: format!("fixture-world-session:{actor_id}"),
            actor_id: actor_id.to_string(),
            reason: "fixture_session_authorized_without_cex_cookie".to_string(),
            source_of_truth: "fixture_world_session_guard".to_string(),
        }
    }
}

impl WorldLedgerAdapter for FixtureWorldRuntimeAdapters {
    fn reserve_reward(&self, route_task_id: &str, amount_units: u64) -> WorldLedgerReceipt {
        WorldLedgerReceipt {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            receipt_id: format!("fixture-ledger-reserve:{route_task_id}"),
            route_task_id: route_task_id.to_string(),
            amount_units,
            status: "reserved_fixture".to_string(),
            source_of_truth: "fixture_world_ledger_adapter".to_string(),
        }
    }

    fn release_reward(&self, receipt_id: &str) -> WorldLedgerReceipt {
        WorldLedgerReceipt {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            receipt_id: receipt_id.to_string(),
            route_task_id: "fixture-route-task".to_string(),
            amount_units: 10,
            status: "released_fixture".to_string(),
            source_of_truth: "fixture_world_ledger_adapter".to_string(),
        }
    }
}

impl WorldRepository for FixtureWorldRuntimeAdapters {
    fn load_world(&self, _actor_id: &str) -> WorldState {
        cex_default_world_from_map_provider()
    }

    fn load_route_records(&self, actor_id: &str, world: &WorldState) -> WorldRouteRecords {
        WorldRouteRecords::fixture_for_split(world, actor_id)
    }

    fn save_world(
        &self,
        world: &WorldState,
        records: &WorldRouteRecords,
    ) -> WorldRepositoryReceipt {
        WorldRepositoryReceipt {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            receipt_id: format!(
                "fixture-repository-save:{}:{}",
                world.nodes.len(),
                records.preview_items().len()
            ),
            state_contract: world.contract_version.clone(),
            route_record_count: records.preview_items().len(),
            status: "saved_fixture_snapshot".to_string(),
            source_of_truth: "fixture_world_repository_adapter".to_string(),
        }
    }
}

impl WorldEvidenceSink for FixtureWorldRuntimeAdapters {
    fn record_evidence(&self, evidence_kind: &str) -> WorldEvidenceReceipt {
        WorldEvidenceReceipt {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            receipt_id: format!("fixture-evidence:{evidence_kind}"),
            evidence_kind: evidence_kind.to_string(),
            status: "recorded_fixture".to_string(),
            source_of_truth: "fixture_world_evidence_sink".to_string(),
        }
    }
}

impl WorldMetricsSink for FixtureWorldRuntimeAdapters {
    fn record_metric(&self, metric_name: &str, value: i64) -> WorldMetricReceipt {
        WorldMetricReceipt {
            adapter_contract: WORLD_RUNTIME_ADAPTER_CONTRACT.to_string(),
            receipt_id: format!("fixture-metric:{metric_name}:{value}"),
            metric_name: metric_name.to_string(),
            value,
            status: "recorded_fixture".to_string(),
            source_of_truth: "fixture_world_metrics_sink".to_string(),
        }
    }
}

pub fn build_home_response(actor_id: &str) -> WorldApiHomeResponse {
    WorldApiHomeResponse::new(
        project_home(&WorldState::fixture(), actor_id),
        fixture_provider_status(),
    )
}

pub fn build_cex_default_home_response(actor_id: &str) -> WorldApiHomeResponse {
    WorldApiHomeResponse::new(
        project_home(&cex_default_world_from_map_provider(), actor_id),
        fixture_provider_status(),
    )
}

pub fn build_home_fragment(actor_id: &str) -> String {
    let state = WorldState::fixture();
    let projection = project_home(&state, actor_id);
    let keypad = projection
        .player_node_id
        .as_deref()
        .map(|node_id| render_keypad_buttons_fragment(&state, node_id))
        .unwrap_or_default();
    format!("{}\n{}", render_home_fragment(&projection), keypad)
}

pub fn apply_fixture_command(command: WorldCommand) -> WorldApiCommandResponse {
    let mut state = WorldState::fixture();
    let decision = apply_command(&mut state, command);
    WorldApiCommandResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        decision,
        state,
    }
}

pub fn build_route_command_target_response(command: &str) -> WorldApiRouteCommandTargetResponse {
    WorldApiRouteCommandTargetResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        target: world_route_command_target(command),
    }
}

pub fn build_route_artifacts_response() -> WorldApiRouteArtifactsResponse {
    let raw_items = vec![
        json!({
            "route_bucket": "delivery",
            "location_id": "delivery-dock",
            "task_id": "task-delivery-1",
            "route_status": "pending",
            "created_at_epoch": 10,
            "title": "Delivery route",
            "summary": "evidence ready with risk controls",
            "detail": "customer deliverable"
        }),
        json!({
            "route_bucket": "rejection",
            "location_id": "delivery-dock",
            "task_id": "task-recovery-1",
            "route_status": "rejected_chargeback_failed",
            "created_at_epoch": 20,
            "title": "Recovery route",
            "summary": "seller chargeback failed",
            "detail": "settlement retry required"
        }),
    ];
    WorldApiRouteArtifactsResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        artifacts: world_route_artifacts_from_raw_preview_items(raw_items, 6),
    }
}

pub fn build_map_runtime_budget_response() -> WorldApiMapRuntimeBudgetResponse {
    WorldApiMapRuntimeBudgetResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        budget: trillionnium_world_map_runtime_performance_budget_json(13, 20, 5, 5, 73),
    }
}

pub fn build_map_pack_manifest_response() -> serde_json::Value {
    trillionnium_world_fixture_map_pack_unsigned_manifest_json()
}

pub fn build_map_pack_attribution_evidence_response() -> serde_json::Value {
    trillionnium_world_map_pack_attribution_evidence_json()
}

pub fn build_map_pack_sensitive_poi_report_response() -> serde_json::Value {
    trillionnium_world_map_pack_sensitive_poi_filter_report_json()
}

pub fn build_map_modeling_gate_response() -> serde_json::Value {
    trillionnium_world_map_modeling_gate_json()
}

pub fn apply_fixture_tactics_command(command: &str) -> WorldApiTacticsCommandResponse {
    let mut character = WorldTrillionniumCharacter::default_for("local-player");
    if command == "attack" {
        let train_request = WorldTacticsCommandRequest {
            command: "train_skill".to_string(),
            unit_id: "lord".to_string(),
            target_tile: None,
            skill_id: Some("basic_unarmed".to_string()),
            npc_id: Some("npc-street-compass-sifu".to_string()),
            item_id: None,
            target_slot: None,
        };
        let _ = apply_tactics_command(&mut character, train_request, 1);
    }
    let request = match command {
        "equip_item" => WorldTacticsCommandRequest {
            command: "equip_item".to_string(),
            unit_id: "lord".to_string(),
            target_tile: None,
            skill_id: None,
            npc_id: None,
            item_id: Some("route-guard-staff".to_string()),
            target_slot: Some("weapon".to_string()),
        },
        "attack" => WorldTacticsCommandRequest {
            command: "attack".to_string(),
            unit_id: "lord".to_string(),
            target_tile: Some("F5".to_string()),
            skill_id: Some("basic_unarmed".to_string()),
            npc_id: None,
            item_id: None,
            target_slot: None,
        },
        "complete_task" => WorldTacticsCommandRequest {
            command: "complete_task".to_string(),
            unit_id: "lord".to_string(),
            target_tile: None,
            skill_id: None,
            npc_id: None,
            item_id: None,
            target_slot: None,
        },
        other => WorldTacticsCommandRequest {
            command: other.to_string(),
            unit_id: "lord".to_string(),
            target_tile: None,
            skill_id: Some("basic_unarmed".to_string()),
            npc_id: Some("npc-street-compass-sifu".to_string()),
            item_id: None,
            target_slot: None,
        },
    };
    WorldApiTacticsCommandResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        outcome: apply_tactics_command(&mut character, request, 2),
    }
}

pub fn build_adapter_readiness_response() -> WorldRuntimeAdapterReadiness {
    world_runtime_adapter_readiness()
}

pub fn build_full_split_response(actor_id: &str) -> WorldApiFullSplitResponse {
    let adapters = FixtureWorldRuntimeAdapters;
    let identity = adapters.resolve_actor(actor_id);
    let session = adapters.authorize_world_session(&identity.actor_id);
    let world = adapters.load_world(&identity.actor_id);
    let records = adapters.load_route_records(&identity.actor_id, &world);
    let repository_receipt = adapters.save_world(&world, &records);
    let ledger_receipt = adapters.reserve_reward("tactics-objective:local-player:first-route", 10);
    let evidence_receipt = adapters.record_evidence("full_split_fixture_projection");
    let metric_receipt = adapters.record_metric("trillionnium_world_full_split_fixture_green", 1);
    let mut projection = world_full_split_projection_json(&world, &identity.actor_id);
    if let Some(object) = projection.as_object_mut() {
        object.insert(
            "identity_adapter".to_string(),
            serde_json::to_value(identity).expect("identity serializes"),
        );
        object.insert(
            "session_guard".to_string(),
            serde_json::to_value(session).expect("session serializes"),
        );
        object.insert(
            "repository_receipt".to_string(),
            serde_json::to_value(repository_receipt).expect("repository receipt serializes"),
        );
        object.insert(
            "ledger_receipt".to_string(),
            serde_json::to_value(ledger_receipt).expect("ledger receipt serializes"),
        );
        object.insert(
            "evidence_receipt".to_string(),
            serde_json::to_value(evidence_receipt).expect("evidence receipt serializes"),
        );
        object.insert(
            "metric_receipt".to_string(),
            serde_json::to_value(metric_receipt).expect("metric receipt serializes"),
        );
    }
    WorldApiFullSplitResponse {
        api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
        response_contract: WORLD_FULL_SPLIT_RESPONSE_CONTRACT.to_string(),
        domain_contract: trnm_world_domain::WORLD_DOMAIN_CONTRACT.to_string(),
        runtime_adapters: build_adapter_readiness_response(),
        projection,
        map_provider: fixture_provider_status(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldDevServerHttpResponse {
    pub status_code: u16,
    pub reason: &'static str,
    pub content_type: &'static str,
    pub body: String,
}

impl WorldDevServerHttpResponse {
    fn json(status_code: u16, reason: &'static str, value: serde_json::Value) -> Self {
        Self {
            status_code,
            reason,
            content_type: "application/json; charset=utf-8",
            body: serde_json::to_string_pretty(&value).expect("dev runtime JSON serializes"),
        }
    }

    fn html(body: String) -> Self {
        Self {
            status_code: 200,
            reason: "OK",
            content_type: "text/html; charset=utf-8",
            body,
        }
    }

    fn not_found(path: &str) -> Self {
        Self::json(
            404,
            "Not Found",
            json!({
                "contract_version": WORLD_DEV_RUNTIME_CONTRACT,
                "status": "not_found",
                "path": path,
            }),
        )
    }

    fn bad_request(message: &str) -> Self {
        Self::json(
            400,
            "Bad Request",
            json!({
                "contract_version": WORLD_DEV_RUNTIME_CONTRACT,
                "status": "bad_request",
                "message": message,
            }),
        )
    }

    fn internal_error(message: &str) -> Self {
        Self::json(
            500,
            "Internal Server Error",
            json!({
                "contract_version": WORLD_DEV_RUNTIME_CONTRACT,
                "status": "internal_error",
                "message": message,
            }),
        )
    }
}

#[derive(Debug)]
pub struct WorldDevRuntime {
    actor_id: String,
    world: Mutex<WorldState>,
    route_records: Mutex<WorldRouteRecords>,
    state_file: Option<PathBuf>,
}

impl WorldDevRuntime {
    pub fn fixture(actor_id: impl Into<String>) -> Self {
        let actor_id = actor_id.into();
        let world = cex_default_world_from_map_provider();
        let route_records = WorldRouteRecords::fixture_for_split(&world, &actor_id);
        Self {
            actor_id,
            world: Mutex::new(world),
            route_records: Mutex::new(route_records),
            state_file: None,
        }
    }

    pub fn file_backed(
        actor_id: impl Into<String>,
        state_file: impl Into<PathBuf>,
        reset_state: bool,
    ) -> std::io::Result<Self> {
        let actor_id = actor_id.into();
        let state_file = state_file.into();
        let world = if state_file.exists() && !reset_state {
            let raw = fs::read_to_string(&state_file)?;
            serde_json::from_str::<WorldState>(&raw).map_err(invalid_data)?
        } else {
            let world = cex_default_world_from_map_provider();
            write_world_state_file(&state_file, &world)?;
            world
        };
        let route_records = WorldRouteRecords::fixture_for_split(&world, &actor_id);
        Ok(Self {
            actor_id,
            world: Mutex::new(world),
            route_records: Mutex::new(route_records),
            state_file: Some(state_file),
        })
    }

    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }

    pub fn health_json(&self) -> serde_json::Value {
        let world = self.world.lock().expect("dev world mutex is not poisoned");
        json!({
            "contract_version": WORLD_DEV_RUNTIME_CONTRACT,
            "server_contract": WORLD_SERVER_CONTRACT,
            "status": "ok",
            "source_of_truth": "rust_world_state_projection_and_command_kernel",
            "runtime_role": "standalone_trnm_world_dev_server",
            "authority_model": "clients_submit_intent_only",
            "repository_contract": WORLD_DEV_REPOSITORY_CONTRACT,
            "repository_mode": if self.state_file.is_some() { "file_backed_json" } else { "in_memory_fixture" },
            "state_file": self.state_file.as_ref().map(|path| path.display().to_string()),
            "actor_id": self.actor_id,
            "node_count": world.nodes.len(),
            "route_count": world.edges.len(),
            "npc_count": world.npcs.len(),
            "task_count": world.tasks.len(),
        })
    }

    pub fn home_response(&self) -> WorldApiHomeResponse {
        let world = self.world.lock().expect("dev world mutex is not poisoned");
        WorldApiHomeResponse::new(
            project_home(&world, &self.actor_id),
            fixture_provider_status(),
        )
    }

    pub fn home_fragment(&self) -> String {
        let world = self.world.lock().expect("dev world mutex is not poisoned");
        let projection = project_home(&world, &self.actor_id);
        let keypad = projection
            .player_node_id
            .as_deref()
            .map(|node_id| render_keypad_buttons_fragment(&world, node_id))
            .unwrap_or_default();
        format!("{}\n{}", render_home_fragment(&projection), keypad)
    }

    pub fn browser_parity_shell(&self) -> String {
        render_browser_parity_shell(&self.actor_id, &self.health_json())
    }

    pub fn command_response(&self, command: WorldCommand) -> WorldApiCommandResponse {
        self.try_command_response(command)
            .expect("dev runtime command persistence succeeds")
    }

    pub fn try_command_response(
        &self,
        command: WorldCommand,
    ) -> std::io::Result<WorldApiCommandResponse> {
        let mut world = self.world.lock().expect("dev world mutex is not poisoned");
        let decision = apply_command(&mut world, command);
        if let Some(state_file) = &self.state_file {
            write_world_state_file(state_file, &world)?;
        }
        Ok(WorldApiCommandResponse {
            api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
            decision,
            state: world.clone(),
        })
    }

    pub fn full_split_response(&self) -> WorldApiFullSplitResponse {
        let adapters = FixtureWorldRuntimeAdapters;
        let identity = adapters.resolve_actor(&self.actor_id);
        let session = adapters.authorize_world_session(&identity.actor_id);
        let world = self.world.lock().expect("dev world mutex is not poisoned");
        let records = self
            .route_records
            .lock()
            .expect("dev route records mutex is not poisoned");
        let repository_receipt = adapters.save_world(&world, &records);
        let ledger_receipt =
            adapters.reserve_reward("tactics-objective:local-player:first-route", 10);
        let evidence_receipt = adapters.record_evidence("dev_runtime_full_split_projection");
        let metric_receipt = adapters.record_metric("trillionnium_world_dev_runtime_green", 1);
        let mut projection = world_full_split_projection_json(&world, &identity.actor_id);
        if let Some(object) = projection.as_object_mut() {
            object.insert(
                "dev_runtime_contract".to_string(),
                json!(WORLD_DEV_RUNTIME_CONTRACT),
            );
            object.insert(
                "identity_adapter".to_string(),
                serde_json::to_value(identity).expect("identity serializes"),
            );
            object.insert(
                "session_guard".to_string(),
                serde_json::to_value(session).expect("session serializes"),
            );
            object.insert(
                "repository_receipt".to_string(),
                serde_json::to_value(repository_receipt).expect("repository receipt serializes"),
            );
            object.insert(
                "ledger_receipt".to_string(),
                serde_json::to_value(ledger_receipt).expect("ledger receipt serializes"),
            );
            object.insert(
                "evidence_receipt".to_string(),
                serde_json::to_value(evidence_receipt).expect("evidence receipt serializes"),
            );
            object.insert(
                "metric_receipt".to_string(),
                serde_json::to_value(metric_receipt).expect("metric receipt serializes"),
            );
        }
        WorldApiFullSplitResponse {
            api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
            response_contract: WORLD_FULL_SPLIT_RESPONSE_CONTRACT.to_string(),
            domain_contract: trnm_world_domain::WORLD_DOMAIN_CONTRACT.to_string(),
            runtime_adapters: build_adapter_readiness_response(),
            projection,
            map_provider: fixture_provider_status(),
        }
    }
}

pub fn build_dev_runtime_smoke_json() -> serde_json::Value {
    let runtime = WorldDevRuntime::fixture("local-player");
    let health = runtime.health_json();
    let home = runtime.home_response();
    let command = runtime.command_response(WorldCommand::Move {
        actor_id: runtime.actor_id().to_string(),
        direction: "east".to_string(),
    });
    let full_split = runtime.full_split_response();
    json!({
        "contract_version": WORLD_DEV_RUNTIME_CONTRACT,
        "status": "dev_runtime_smoke_ready",
        "source_of_truth": "rust_world_state_projection_and_command_kernel",
        "server_contract": WORLD_SERVER_CONTRACT,
        "endpoints": [
            "/health",
            "/world/home",
            "/world/home-fragment",
            "/world/play",
            "/world/state",
            "/world/command",
            "/world/tactics-command",
            "/world/full-split",
            "/world/adapter-readiness",
            "/world/map-runtime-budget",
            "/world/route-artifacts"
        ],
        "health": health,
        "home_node_count": home.home.node_count,
        "command_accepted": command.decision.accepted,
        "command_player_node": command
            .state
            .positions
            .iter()
            .find(|position| position.actor_id == runtime.actor_id())
            .map(|position| position.node_id.clone())
            .unwrap_or_default(),
        "full_split_contract": full_split.response_contract,
        "runtime_adapter_count": full_split.runtime_adapters.statuses.len(),
    })
}

pub fn build_dev_runtime_repository_smoke_json(
    state_file: impl Into<PathBuf>,
) -> std::io::Result<serde_json::Value> {
    let state_file = state_file.into();
    let runtime = WorldDevRuntime::file_backed("local-player", state_file.clone(), true)?;
    let before = runtime.health_json();
    let command = runtime.try_command_response(WorldCommand::Move {
        actor_id: runtime.actor_id().to_string(),
        direction: "east".to_string(),
    })?;
    let reloaded = WorldDevRuntime::file_backed("local-player", state_file.clone(), false)?;
    let reloaded_home = reloaded.home_response();
    let reloaded_node = reloaded_home
        .home
        .player_node_id
        .clone()
        .unwrap_or_default();
    Ok(json!({
        "contract_version": WORLD_DEV_REPOSITORY_CONTRACT,
        "dev_runtime_contract": WORLD_DEV_RUNTIME_CONTRACT,
        "status": if reloaded_node == "starter-studio" { "file_repository_persistence_green" } else { "file_repository_persistence_failed" },
        "state_file": state_file.display().to_string(),
        "state_file_exists": state_file.exists(),
        "source_of_truth": "rust_world_state_json_repository",
        "before_repository_mode": before["repository_mode"],
        "command_accepted": command.decision.accepted,
        "command_player_node": player_node(&command.state, runtime.actor_id()),
        "reloaded_player_node": reloaded_node,
        "reloaded_node_count": reloaded_home.home.node_count,
    }))
}

pub fn handle_dev_runtime_request(
    runtime: &WorldDevRuntime,
    method: &str,
    raw_path: &str,
    body: &str,
) -> WorldDevServerHttpResponse {
    let (path, query) = split_path_query(raw_path);
    match (method, path.as_str()) {
        ("GET", "/health") => WorldDevServerHttpResponse::json(200, "OK", runtime.health_json()),
        ("GET", "/world/home") => json_response(&runtime.home_response()),
        ("GET", "/world/home-fragment") => {
            WorldDevServerHttpResponse::html(runtime.home_fragment())
        }
        ("GET", "/world/play") | ("GET", "/world/browser-parity") => {
            WorldDevServerHttpResponse::html(runtime.browser_parity_shell())
        }
        ("GET", "/world/state") => {
            let world = runtime
                .world
                .lock()
                .expect("dev world mutex is not poisoned");
            json_response(&*world)
        }
        ("GET", "/world/full-split") => json_response(&runtime.full_split_response()),
        ("GET", "/world/adapter-readiness") => json_response(&build_adapter_readiness_response()),
        ("GET", "/world/map-runtime-budget") => json_response(&build_map_runtime_budget_response()),
        ("GET", "/world/route-artifacts") => json_response(&build_route_artifacts_response()),
        ("GET", "/world/tactics-command") => {
            let command = query
                .get("command")
                .map(String::as_str)
                .unwrap_or("train_skill");
            json_response(&apply_fixture_tactics_command(command))
        }
        ("GET", "/world/command") => {
            let actor_id = query
                .get("actor_id")
                .cloned()
                .unwrap_or_else(|| runtime.actor_id().to_string());
            let direction = query
                .get("direction")
                .cloned()
                .unwrap_or_else(|| "east".to_string());
            match runtime.try_command_response(WorldCommand::Move {
                actor_id,
                direction,
            }) {
                Ok(response) => json_response(&response),
                Err(error) => WorldDevServerHttpResponse::internal_error(&error.to_string()),
            }
        }
        ("POST", "/world/command") => match parse_world_command_request(body, runtime.actor_id()) {
            Ok(command) => match runtime.try_command_response(command) {
                Ok(response) => json_response(&response),
                Err(error) => WorldDevServerHttpResponse::internal_error(&error.to_string()),
            },
            Err(error) => WorldDevServerHttpResponse::bad_request(&error),
        },
        _ => WorldDevServerHttpResponse::not_found(&path),
    }
}

pub fn serve_dev_runtime(
    bind_addr: &str,
    actor_id: &str,
    state_file: Option<PathBuf>,
    reset_state: bool,
) -> std::io::Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    let runtime = if let Some(state_file) = state_file {
        WorldDevRuntime::file_backed(actor_id, state_file, reset_state)?
    } else {
        WorldDevRuntime::fixture(actor_id)
    };
    for stream in listener.incoming() {
        let mut stream = stream?;
        if let Err(error) = handle_stream(&runtime, &mut stream) {
            let response = WorldDevServerHttpResponse::internal_error(&error.to_string());
            write_http_response(&mut stream, &response)?;
        }
    }
    Ok(())
}

fn json_response<T: serde::Serialize>(value: &T) -> WorldDevServerHttpResponse {
    WorldDevServerHttpResponse {
        status_code: 200,
        reason: "OK",
        content_type: "application/json; charset=utf-8",
        body: serde_json::to_string_pretty(value).expect("dev runtime response serializes"),
    }
}

fn render_browser_parity_shell(actor_id: &str, health: &serde_json::Value) -> String {
    let runtime_contract = health["contract_version"].as_str().unwrap_or_default();
    let repository_mode = health["repository_mode"].as_str().unwrap_or_default();
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>Trillionnium World Browser Parity</title>
  <style>
    :root {{
      color-scheme: dark;
      --bg: #111612;
      --panel: #192019;
      --panel-2: #202920;
      --line: #344136;
      --text: #edf7ee;
      --muted: #aebcaf;
      --accent: #8bd17c;
      --accent-2: #f1c65b;
      --danger: #ef7c74;
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      min-height: 100vh;
      background: var(--bg);
      color: var(--text);
      font: 15px/1.45 system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }}
    main {{
      width: min(980px, 100%);
      margin: 0 auto;
      padding: 18px;
      display: grid;
      gap: 14px;
    }}
    header, section {{
      border: 1px solid var(--line);
      background: var(--panel);
      border-radius: 8px;
      padding: 14px;
    }}
    h1, h2 {{
      margin: 0 0 10px;
      font-size: 18px;
      letter-spacing: 0;
    }}
    h2 {{ font-size: 15px; color: var(--muted); }}
    dl {{
      display: grid;
      grid-template-columns: max-content 1fr;
      gap: 8px 12px;
      margin: 0;
    }}
    dt {{ color: var(--muted); }}
    dd {{ margin: 0; overflow-wrap: anywhere; }}
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
      gap: 14px;
    }}
    .actions {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(128px, 1fr));
      gap: 8px;
    }}
    button {{
      min-height: 42px;
      border: 1px solid var(--line);
      border-radius: 8px;
      background: var(--panel-2);
      color: var(--text);
      font: inherit;
      cursor: pointer;
    }}
    button.primary {{ border-color: var(--accent); color: var(--accent); }}
    button:focus-visible {{ outline: 2px solid var(--accent-2); outline-offset: 2px; }}
    pre {{
      min-height: 90px;
      max-height: 260px;
      overflow: auto;
      margin: 0;
      padding: 12px;
      background: #0c100d;
      border: 1px solid var(--line);
      border-radius: 8px;
      white-space: pre-wrap;
      overflow-wrap: anywhere;
    }}
    .ok {{ color: var(--accent); }}
    .fail {{ color: var(--danger); }}
  </style>
</head>
<body>
  <main id="world-browser-parity-shell"
    data-contract="{contract}"
    data-render-owner="rust_world_browser_parity_shell"
    data-client-role="intent_only_browser_client">
    <header>
      <h1>Trillionnium World</h1>
      <dl>
        <dt>Shell</dt><dd id="browser-contract">{contract}</dd>
        <dt>Runtime</dt><dd id="runtime-contract">{runtime_contract}</dd>
        <dt>Repository</dt><dd id="repository-mode">{repository_mode}</dd>
        <dt>Actor</dt><dd id="actor-id">{actor_id}</dd>
      </dl>
    </header>
    <div class="grid">
      <section>
        <h2>World State</h2>
        <dl>
          <dt>Player node</dt><dd id="player-node">loading</dd>
          <dt>Node count</dt><dd id="node-count">loading</dd>
          <dt>Task count</dt><dd id="task-count">loading</dd>
        </dl>
      </section>
      <section>
        <h2>Movement</h2>
        <div class="actions">
          <button id="move-east" class="primary" type="button" data-command="move-east">Move East</button>
          <button id="move-west" type="button" data-command="move-west">Move West</button>
        </div>
      </section>
      <section>
        <h2>Tactics / Task</h2>
        <div class="actions">
          <button id="train-skill" type="button" data-command="train_skill">Train</button>
          <button id="attack" type="button" data-command="attack">Attack</button>
          <button id="offer-task" type="button" data-command="offer_task">Task</button>
          <button id="complete-task" type="button" data-command="complete_task">Complete</button>
        </div>
      </section>
    </div>
    <section>
      <h2>Browser Evidence</h2>
      <dl>
        <dt>Last action</dt><dd id="last-action">none</dd>
        <dt>Last result</dt><dd id="last-result">none</dd>
        <dt>Status</dt><dd id="browser-status" class="ok">booting</dd>
      </dl>
    </section>
    <section>
      <h2>Response</h2>
      <pre id="response-log"></pre>
    </section>
  </main>
  <script>
    window.__trnmWorldBrowserParity = {{
      contract: "{contract}",
      runtimeContract: "{runtime_contract}",
      repositoryMode: "{repository_mode}",
      actorId: "{actor_id}",
      actions: []
    }};
    const actorId = "{actor_id}";
    const logEl = document.getElementById("response-log");
    const statusEl = document.getElementById("browser-status");
    const lastActionEl = document.getElementById("last-action");
    const lastResultEl = document.getElementById("last-result");

    async function getJson(path) {{
      const response = await fetch(path, {{ headers: {{ "Accept": "application/json" }} }});
      if (!response.ok) throw new Error(path + " returned " + response.status);
      return response.json();
    }}

    function playerNodeFromState(state) {{
      const position = (state.positions || []).find((entry) => entry.actor_id === actorId);
      return position ? position.node_id : "";
    }}

    function record(action, payload) {{
      const outcome = payload.outcome || payload.decision || payload;
      const result = outcome.result || outcome.reason || (outcome.accepted === true ? "accepted" : "ok");
      window.__trnmWorldBrowserParity.actions.push({{ action, result, accepted: outcome.accepted !== false }});
      lastActionEl.textContent = action;
      lastResultEl.textContent = result;
      logEl.textContent = JSON.stringify(payload, null, 2);
      statusEl.textContent = "ready";
      statusEl.className = "ok";
    }}

    async function refreshHome() {{
      const home = await getJson("/world/home");
      document.getElementById("player-node").textContent = home.home.player_node_id || "";
      document.getElementById("node-count").textContent = String(home.home.node_count || 0);
      document.getElementById("task-count").textContent = String(home.home.task_count || 0);
      record("home", home);
      return home;
    }}

    async function refreshState() {{
      const state = await getJson("/world/state");
      const playerNode = playerNodeFromState(state);
      document.getElementById("player-node").textContent = playerNode;
      window.__trnmWorldBrowserParity.lastState = {{
        playerNodeId: playerNode,
        nodeCount: (state.nodes || []).length,
        taskCount: (state.tasks || []).length
      }};
      return state;
    }}

    async function move(direction) {{
      statusEl.textContent = "moving";
      const payload = await getJson("/world/command?direction=" + encodeURIComponent(direction) + "&actor_id=" + encodeURIComponent(actorId));
      document.getElementById("player-node").textContent = playerNodeFromState(payload.state || {{}});
      record("move_" + direction, payload);
      await refreshState();
    }}

    async function tactics(command) {{
      statusEl.textContent = command;
      const payload = await getJson("/world/tactics-command?command=" + encodeURIComponent(command));
      record(command, payload);
    }}

    document.getElementById("move-east").addEventListener("click", () => move("east").catch(showError));
    document.getElementById("move-west").addEventListener("click", () => move("west").catch(showError));
    document.getElementById("train-skill").addEventListener("click", () => tactics("train_skill").catch(showError));
    document.getElementById("attack").addEventListener("click", () => tactics("attack").catch(showError));
    document.getElementById("offer-task").addEventListener("click", () => tactics("offer_task").catch(showError));
    document.getElementById("complete-task").addEventListener("click", () => tactics("complete_task").catch(showError));

    function showError(error) {{
      statusEl.textContent = error.message;
      statusEl.className = "fail";
      window.__trnmWorldBrowserParity.error = error.message;
      throw error;
    }}

    refreshHome().then(refreshState).catch(showError);
  </script>
</body>
</html>"#,
        contract = WORLD_BROWSER_PARITY_SHELL_CONTRACT,
        runtime_contract = html_escape(runtime_contract),
        repository_mode = html_escape(repository_mode),
        actor_id = html_escape(actor_id),
    )
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn write_world_state_file(path: &Path, world: &WorldState) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(world).map_err(invalid_data)?;
    fs::write(path, body)
}

fn invalid_data(error: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, error)
}

fn player_node(world: &WorldState, actor_id: &str) -> String {
    world
        .positions
        .iter()
        .find(|position| position.actor_id == actor_id)
        .map(|position| position.node_id.clone())
        .unwrap_or_default()
}

fn parse_world_command_request(body: &str, default_actor_id: &str) -> Result<WorldCommand, String> {
    if body.trim().is_empty() {
        return Ok(WorldCommand::Move {
            actor_id: default_actor_id.to_string(),
            direction: "east".to_string(),
        });
    }
    if let Ok(request) = serde_json::from_str::<WorldApiCommandRequest>(body) {
        return Ok(request.command);
    }
    serde_json::from_str::<WorldCommand>(body).map_err(|error| error.to_string())
}

fn split_path_query(raw_path: &str) -> (String, HashMap<String, String>) {
    let mut parts = raw_path.splitn(2, '?');
    let path = parts.next().unwrap_or("/").to_string();
    let query = parts.next().map(parse_query).unwrap_or_default();
    (path, query)
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut kv = part.splitn(2, '=');
            let key = percent_decode(kv.next().unwrap_or_default());
            let value = percent_decode(kv.next().unwrap_or_default());
            (key, value)
        })
        .collect()
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    output.push(byte);
                    index += 3;
                } else {
                    output.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                output.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&output).into_owned()
}

fn handle_stream(runtime: &WorldDevRuntime, stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 16 * 1024];
    let read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let (head, body) = request.split_once("\r\n\r\n").unwrap_or((&request, ""));
    let mut lines = head.lines();
    let request_line = lines.next().unwrap_or_default();
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or("GET");
    let path = request_parts.next().unwrap_or("/");
    let response = handle_dev_runtime_request(runtime, method, path, body);
    write_http_response(stream, &response)
}

fn write_http_response(
    stream: &mut TcpStream,
    response: &WorldDevServerHttpResponse,
) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response.status_code,
        response.reason,
        response.content_type,
        response.body.len(),
        response.body
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_world_command::WorldCommand;

    #[test]
    fn standalone_home_response_is_available() {
        let response = build_home_response("local-player");
        assert_eq!(
            response.home.player_node_id.as_deref(),
            Some("mirror-city-square")
        );
    }

    #[test]
    fn standalone_cex_default_home_response_uses_full_map_fixture() {
        let response = build_cex_default_home_response("local-player");
        assert_eq!(response.home.node_count, 24);
        assert_eq!(
            response.home.player_node_id.as_deref(),
            Some("mirror-city-square")
        );
    }

    #[test]
    fn standalone_command_response_mutates_fixture() {
        let response = apply_fixture_command(WorldCommand::Move {
            actor_id: "local-player".to_string(),
            direction: "east".to_string(),
        });
        assert!(response.decision.accepted);
        assert_eq!(
            response
                .state
                .positions
                .iter()
                .find(|position| position.actor_id == "local-player")
                .unwrap()
                .node_id,
            "league-coliseum"
        );
    }

    #[test]
    fn standalone_route_command_target_response_is_available() {
        let response = build_route_command_target_response("/work reject latest 重试拒收退款");
        assert_eq!(response.api_contract, trnm_world_api::WORLD_API_CONTRACT);
        assert_eq!(response.target.panel_id, "world-commerce-panel");
        assert_eq!(response.target.input_id, "world-work-reject-id");
    }

    #[test]
    fn standalone_route_artifacts_response_is_available() {
        let response = build_route_artifacts_response();
        assert_eq!(response.api_contract, trnm_world_api::WORLD_API_CONTRACT);
        assert_eq!(response.artifacts.task_views.len(), 2);
        assert_eq!(
            response.artifacts.story.next_opportunity_kind,
            "rejection_chargeback_recovery"
        );
        assert_eq!(
            response.artifacts.story.next_opportunity_target.input_id,
            "world-work-reject-id"
        );
    }

    #[test]
    fn standalone_map_runtime_budget_response_is_available() {
        let response = build_map_runtime_budget_response();
        assert_eq!(response.api_contract, trnm_world_api::WORLD_API_CONTRACT);
        assert_eq!(
            response.budget["contract_version"],
            trnm_world_map_provider::TRILLIONNIUM_WORLD_MAP_RUNTIME_PERFORMANCE_BUDGET_CONTRACT_VERSION
        );
    }

    #[test]
    fn standalone_map_pack_evidence_responses_are_available() {
        let manifest = build_map_pack_manifest_response();
        assert_eq!(
            manifest["contract_version"],
            trnm_world_map_provider::WORLD_MAP_PACK_MANIFEST_CONTRACT
        );
        assert_eq!(manifest["node_count"], 24);

        let attribution = build_map_pack_attribution_evidence_response();
        assert_eq!(
            attribution["contract_version"],
            trnm_world_map_provider::WORLD_MAP_PACK_ATTRIBUTION_EVIDENCE_CONTRACT
        );

        let sensitive = build_map_pack_sensitive_poi_report_response();
        assert_eq!(sensitive["flagged_node_count"], 0);

        let modeling = build_map_modeling_gate_response();
        assert_eq!(
            modeling["contract_version"],
            trnm_world_map_provider::TRILLIONNIUM_WORLD_MAP_MODELING_GATE_CONTRACT_VERSION
        );
        assert_eq!(modeling["gates"]["all_layers_modeled"], true);
        assert_eq!(modeling["runtime_clients_fetch_public_osm_directly"], false);
    }

    #[test]
    fn standalone_tactics_command_response_mutates_character() {
        let train = apply_fixture_tactics_command("train_skill");
        assert!(train.outcome.accepted);
        assert_eq!(train.outcome.result, "skill_training_recorded");
        assert!(train
            .outcome
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "basic_unarmed"));

        let equip = apply_fixture_tactics_command("equip_item");
        assert_eq!(equip.outcome.result, "item_equipped");
        assert_eq!(equip.outcome.equipped_slot.as_deref(), Some("weapon"));
    }

    #[test]
    fn standalone_full_split_response_uses_fixture_runtime_adapters() {
        let response = build_full_split_response("local-player");
        assert_eq!(
            response.response_contract,
            WORLD_FULL_SPLIT_RESPONSE_CONTRACT
        );
        assert_eq!(response.runtime_adapters.statuses.len(), 6);
        assert_eq!(
            response.projection["cex_dependency_status"],
            "no_trnm_world_crate_depends_on_cex_service_internals"
        );
        assert_eq!(
            response.projection["identity_adapter"]["source_of_truth"],
            "fixture_world_identity_adapter"
        );
        assert_eq!(
            response.projection["ledger_receipt"]["status"],
            "reserved_fixture"
        );
    }

    #[test]
    fn dev_runtime_health_and_command_endpoints_are_available() {
        let runtime = WorldDevRuntime::fixture("local-player");
        let health = handle_dev_runtime_request(&runtime, "GET", "/health", "");
        assert_eq!(health.status_code, 200);
        assert!(health.body.contains(WORLD_DEV_RUNTIME_CONTRACT));

        let command = handle_dev_runtime_request(
            &runtime,
            "GET",
            "/world/command?direction=east&actor_id=local-player",
            "",
        );
        assert_eq!(command.status_code, 200);
        assert!(command.body.contains("\"accepted\": true"));
        assert!(command.body.contains("starter-studio"));

        let home = handle_dev_runtime_request(&runtime, "GET", "/world/home", "");
        assert_eq!(home.status_code, 200);
        assert!(home.body.contains("\"player_node_id\": \"starter-studio\""));
    }

    #[test]
    fn dev_runtime_browser_parity_shell_is_available() {
        let runtime = WorldDevRuntime::fixture("local-player");
        let response = handle_dev_runtime_request(&runtime, "GET", "/world/play", "");
        assert_eq!(response.status_code, 200);
        assert_eq!(response.content_type, "text/html; charset=utf-8");
        assert!(response.body.contains(WORLD_BROWSER_PARITY_SHELL_CONTRACT));
        assert!(response
            .body
            .contains("data-client-role=\"intent_only_browser_client\""));
        assert!(response.body.contains("id=\"move-east\""));
        assert!(response.body.contains("id=\"train-skill\""));
        assert!(response.body.contains("/world/state"));
        assert!(response.body.contains("/world/tactics-command"));
    }

    #[test]
    fn dev_runtime_post_command_accepts_api_request_json() {
        let runtime = WorldDevRuntime::fixture("local-player");
        let body = serde_json::to_string(&WorldApiCommandRequest {
            api_contract: trnm_world_api::WORLD_API_CONTRACT.to_string(),
            command: WorldCommand::Move {
                actor_id: "local-player".to_string(),
                direction: "north".to_string(),
            },
        })
        .unwrap();
        let response = handle_dev_runtime_request(&runtime, "POST", "/world/command", &body);
        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("league-coliseum"));
    }

    #[test]
    fn dev_runtime_smoke_json_carries_runtime_contract() {
        let smoke = build_dev_runtime_smoke_json();
        assert_eq!(smoke["contract_version"], WORLD_DEV_RUNTIME_CONTRACT);
        assert_eq!(smoke["status"], "dev_runtime_smoke_ready");
        assert_eq!(smoke["command_player_node"], "starter-studio");
        assert_eq!(smoke["runtime_adapter_count"], 6);
    }

    #[test]
    fn dev_runtime_file_repository_persists_world_state() {
        let path = std::env::temp_dir().join(format!(
            "trnm-world-dev-runtime-state-{}.json",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let smoke = build_dev_runtime_repository_smoke_json(path.clone()).unwrap();
        assert_eq!(smoke["contract_version"], WORLD_DEV_REPOSITORY_CONTRACT);
        assert_eq!(smoke["status"], "file_repository_persistence_green");
        assert_eq!(smoke["command_player_node"], "starter-studio");
        assert_eq!(smoke["reloaded_player_node"], "starter-studio");
        assert!(path.exists());
        let _ = std::fs::remove_file(path);
    }
}
