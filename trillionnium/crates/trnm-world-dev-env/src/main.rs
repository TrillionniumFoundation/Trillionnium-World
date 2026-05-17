fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&trnm_world_dev_env::report())
            .expect("dev env report serializes")
    );
}
