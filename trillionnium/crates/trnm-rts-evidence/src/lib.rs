//! Bevy-free RTS evidence summaries and gates.
//!
//! This crate keeps proof contracts separate from `trnm-world-bevy` rendering code.

use serde::{Deserialize, Serialize};
use trnm_rts_bevy_runtime::{
    rts_command_queue_path_preview_stage, rts_minimap_cell_origin, rts_runtime_hit_test_grid,
    rts_runtime_tile_line, RtsRuntimeGridSpec, RtsRuntimeTileLineStep,
    TRNM_RTS_BEVY_RUNTIME_CONTRACT,
};

pub const TRNM_RTS_EVIDENCE_CONTRACT: &str = "trnm_rts_evidence_v1";
pub const TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT: &str =
    "trnm_rts_evidence_bevy_runtime_adapter_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsEvidencePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsBevyRuntimeAdapterEvidence {
    pub contract_version: String,
    pub runtime_contract: String,
    pub green: bool,
    pub minimap_cell_sample: RtsEvidencePoint,
    pub path_preview_sample: Option<String>,
    pub command_grid_hit_sample: Option<usize>,
    pub tile_line_sample: Vec<RtsRuntimeTileLineStep>,
    pub source_of_truth: String,
}

pub fn first_contact_bevy_runtime_adapter_evidence() -> RtsBevyRuntimeAdapterEvidence {
    let minimap_cell = rts_minimap_cell_origin(10, 20, 4, 5, (32, 32));
    let preview_queue = vec!["command_queue_path_preview:queue_stack".to_string()];
    let path_preview =
        rts_command_queue_path_preview_stage(&[], &preview_queue, 0).map(str::to_string);
    let command_grid_hit = rts_runtime_hit_test_grid(
        RtsRuntimeGridSpec {
            origin_x: 360,
            origin_y: 572,
            columns: 6,
            count: 12,
            stride_x: 58,
            stride_y: 46,
            slot_width: 48,
            slot_height: 38,
        },
        363,
        575,
    );
    let tile_line = rts_runtime_tile_line((8, 8), (12, 16));
    let green = TRNM_RTS_BEVY_RUNTIME_CONTRACT == "trnm_rts_bevy_runtime_adapter_v1"
        && minimap_cell == (134, 175)
        && path_preview.as_deref() == Some("queue_stack")
        && command_grid_hit == Some(0)
        && tile_line.len() == 9
        && tile_line.first().is_some_and(|step| {
            step.step_index == 0 && step.step_count == 8 && step.tile_x == 8 && step.tile_y == 8
        })
        && tile_line.get(4).is_some_and(|step| {
            step.step_index == 4 && step.step_count == 8 && step.tile_x == 10 && step.tile_y == 12
        })
        && tile_line.last().is_some_and(|step| {
            step.step_index == 8 && step.step_count == 8 && step.tile_x == 12 && step.tile_y == 16
        });

    RtsBevyRuntimeAdapterEvidence {
        contract_version: TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT.to_string(),
        runtime_contract: TRNM_RTS_BEVY_RUNTIME_CONTRACT.to_string(),
        green,
        minimap_cell_sample: RtsEvidencePoint {
            x: minimap_cell.0,
            y: minimap_cell.1,
        },
        path_preview_sample: path_preview,
        command_grid_hit_sample: command_grid_hit,
        tile_line_sample: tile_line,
        source_of_truth: "The RTS evidence crate verifies the Bevy-free runtime adapter contract using deterministic First Contact minimap, path preview, command-grid, and tile-line raster samples before trnm-world-bevy includes the proof in release-review evidence.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_runtime_adapter_evidence_is_green() {
        let evidence = first_contact_bevy_runtime_adapter_evidence();

        assert_eq!(
            evidence.contract_version,
            TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT
        );
        assert_eq!(evidence.runtime_contract, TRNM_RTS_BEVY_RUNTIME_CONTRACT);
        assert!(evidence.green);
        assert_eq!(
            evidence.minimap_cell_sample,
            RtsEvidencePoint { x: 134, y: 175 }
        );
        assert_eq!(evidence.path_preview_sample.as_deref(), Some("queue_stack"));
        assert_eq!(evidence.command_grid_hit_sample, Some(0));
        assert_eq!(evidence.tile_line_sample.len(), 9);
        assert_eq!(evidence.tile_line_sample[4].tile_x, 10);
        assert_eq!(evidence.tile_line_sample[4].tile_y, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_x, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_y, 16);
    }
}
