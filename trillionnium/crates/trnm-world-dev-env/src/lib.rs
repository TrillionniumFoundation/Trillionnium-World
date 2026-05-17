//! Dev-environment readiness model for Trillionnium World.

use serde::{Deserialize, Serialize};
use trnm_world_bevy::TRILLIONNIUM_WORLD_BEVY_NATIVE_BRIDGE_CONTRACT;
use trnm_world_domain::{WORLD_CEX_INCUBATOR_SOURCE, WORLD_DOMAIN_CONTRACT};
use trnm_world_map_provider::fixture_provider_status;
use trnm_world_projection::WORLD_PROJECTION_CONTRACT;

pub const WORLD_DEV_ENV_CONTRACT: &str = "trillionnium_world_dev_environment_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldDevEnvReport {
    pub contract_version: String,
    pub cex_incubator_source: String,
    pub domain_contract: String,
    pub projection_contract: String,
    pub map_provider: String,
    pub engine_primary: String,
    pub engine_runtime: String,
    pub native_bevy_gate: String,
    pub bevy_native_client_contract: String,
    pub bevy_native_client_gate: String,
    pub android_target: String,
    pub android_compile_gate: String,
    pub android_native_lib_gate: String,
    pub android_apk_gate: String,
    pub android_real_device_gate: String,
    pub s5_device_evidence_contract: String,
    pub s5_device_evidence_path: String,
    pub device_matrix_gate: String,
    pub godot_gate: String,
    pub map_pack_gate: String,
    pub cell_aoi_gate: String,
    pub quic_h3_gate: String,
}

pub fn report() -> WorldDevEnvReport {
    WorldDevEnvReport {
        contract_version: WORLD_DEV_ENV_CONTRACT.to_string(),
        cex_incubator_source: WORLD_CEX_INCUBATOR_SOURCE.to_string(),
        domain_contract: WORLD_DOMAIN_CONTRACT.to_string(),
        projection_contract: WORLD_PROJECTION_CONTRACT.to_string(),
        map_provider: fixture_provider_status().active_provider,
        engine_primary: "Native/Bevy".to_string(),
        engine_runtime: "bevy_native_client_v1".to_string(),
        native_bevy_gate: "bevy_native_client_green_pending_device_matrix".to_string(),
        bevy_native_client_contract: TRILLIONNIUM_WORLD_BEVY_NATIVE_BRIDGE_CONTRACT.to_string(),
        bevy_native_client_gate: "bevy_client_consumes_rust_projection_and_submits_intent_only"
            .to_string(),
        android_target: "aarch64-linux-android".to_string(),
        android_compile_gate: "cargo_check_trnm_world_bevy_aarch64_linux_android".to_string(),
        android_native_lib_gate: "cdylib_exports_anativeactivity_oncreate_and_android_main"
            .to_string(),
        android_apk_gate: "signed_debug_apk_ready_when_android_platform_jar_available".to_string(),
        android_real_device_gate: "adb_install_launch_screenshot_gfxinfo_logcat_lifecycle"
            .to_string(),
        s5_device_evidence_contract: "trillionnium_world_s5_native_bevy_device_evidence_v1"
            .to_string(),
        s5_device_evidence_path: "acceptance/S5_native_bevy_device/latest/s5-device-evidence.json"
            .to_string(),
        device_matrix_gate: "pending_real_android_device_fps_input_lifecycle_crash_evidence"
            .to_string(),
        godot_gate: "reference_tooling_only".to_string(),
        map_pack_gate: "fixture_only_until_signed_manifest".to_string(),
        cell_aoi_gate: "future_multi_node_gate".to_string(),
        quic_h3_gate: "future_transport_adr_gate".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_is_explicit_about_future_gates() {
        let report = report();
        assert_eq!(report.contract_version, WORLD_DEV_ENV_CONTRACT);
        assert!(report.native_bevy_gate.contains("bevy_native_client_green"));
        assert_eq!(report.engine_primary, "Native/Bevy");
        assert_eq!(report.android_target, "aarch64-linux-android");
        assert!(
            report.android_native_lib_gate.contains("ANativeActivity")
                || report.android_native_lib_gate.contains("anativeactivity")
        );
        assert_eq!(
            report.s5_device_evidence_contract,
            "trillionnium_world_s5_native_bevy_device_evidence_v1"
        );
        assert_eq!(
            report.bevy_native_client_contract,
            TRILLIONNIUM_WORLD_BEVY_NATIVE_BRIDGE_CONTRACT
        );
        assert!(report.map_pack_gate.contains("fixture_only"));
    }
}
