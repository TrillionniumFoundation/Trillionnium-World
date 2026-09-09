#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};
    use tempfile::tempdir;
    use trnm_campaign_core::{
        BattleMapNodeV1, BattleMapSeedV1, CampaignMission, CampaignRoom, CampaignSaveV1,
        MissionDefinition, QuestState,
    };
    use trnm_rts_protocol::{RtsOrderSource, RtsTile};

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/rts_tests_01.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/rts_tests_02.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/rts_tests_03.rs"));
}
