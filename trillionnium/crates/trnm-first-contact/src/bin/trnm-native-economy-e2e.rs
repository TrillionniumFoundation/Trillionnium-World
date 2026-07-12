fn main() {
    let phase = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "verify".to_string());
    let evidence = trnm_first_contact::run_native_economy_e2e_phase(&phase)
        .expect("native Bevy economy E2E phase must succeed");
    println!("{}", serde_json::to_string(&evidence).unwrap());
}
