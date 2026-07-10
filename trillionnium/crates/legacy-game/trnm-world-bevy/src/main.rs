#[cfg(feature = "legacy")]
mod legacy_cli;

#[cfg(feature = "legacy")]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).filter(|arg| arg != "--").collect();
    if args.is_empty()
        || matches!(
            args.first().map(String::as_str),
            Some("run" | "--run" | "play")
        )
    {
        trnm_world_bevy::run_native_bevy_client(
            trnm_world_bevy::native_bevy_playable_fixture(),
            "local-player",
        );
        return;
    }
    legacy_cli::run();
}

#[cfg(not(feature = "legacy"))]
fn main() {
    eprintln!("trnm-world-bevy is frozen; forwarding to the canonical trnm-first-contact client");
    let mut app = trnm_world_bevy::build_first_contact_live_bevy_app(false)
        .expect("canonical First Contact client loads");
    app.run();
}
