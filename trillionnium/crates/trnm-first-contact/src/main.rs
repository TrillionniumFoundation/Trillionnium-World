fn env_enabled(key: &str) -> bool {
    std::env::var(key)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

fn main() {
    let low_spec = env_enabled("TRNM_WORLD_BEVY_LOW_SPEC");
    let mut app = trnm_first_contact::build_first_contact_live_bevy_app(low_spec)
        .expect("authored First Contact map and atlas load for the Bevy player client");
    app.run();
}
