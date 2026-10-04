use super::*;

struct FakeTerminalOrphanAuthority {
    recovered: Option<PublishedTickHighWater>,
    marker: Mutex<Option<PublishedTickHighWater>>,
    fail_acknowledgement: bool,
    durably_failed_closed: bool,
}

impl TerminalOrphanAuthority for FakeTerminalOrphanAuthority {
    async fn recover_running_high_water(
        &self,
        _high_water: &PublishedTickHighWater,
    ) -> Result<Option<PublishedTickHighWater>, String> {
        Ok(self.recovered.clone())
    }

    async fn acknowledge_terminal_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<bool, String> {
        if self.fail_acknowledgement {
            return Err("injected terminal marker failure".to_string());
        }
        *self.marker.lock().expect("fake marker lock") = Some(high_water.clone());
        Ok(true)
    }

    async fn reconcile_failed_closed_high_water(
        &self,
        _high_water: &PublishedTickHighWater,
    ) -> Result<bool, String> {
        Ok(self.durably_failed_closed)
    }
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/lib_parts/tests/game_server_tests_01.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/lib_parts/tests/game_server_tests_02.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/lib_parts/tests/game_server_tests_03.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/lib_parts/tests/game_server_tests_04.rs"
));
