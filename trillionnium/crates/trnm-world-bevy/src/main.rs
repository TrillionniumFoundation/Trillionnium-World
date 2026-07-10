mod legacy_cli;

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
