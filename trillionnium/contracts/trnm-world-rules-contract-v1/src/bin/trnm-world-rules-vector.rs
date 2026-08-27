use trnm_world_rules_contract_v1::{
    execute_transition_verified, EngineOutput, ResourceBudget, TransitionFailure, TransitionRequest,
    WorldRulesEngine,
};

struct VectorEngine;

impl WorldRulesEngine for VectorEngine {
    fn supports(&self, ruleset_revision: &str, content_revision: &str) -> bool {
        ruleset_revision == "first_contact_v1" && content_revision == "content_2026_08_27"
    }

    fn execute(&self, request: &TransitionRequest) -> Result<EngineOutput, TransitionFailure> {
        let mut state_after = request.state_canonical.clone();
        state_after.extend_from_slice(b"|");
        state_after.extend_from_slice(&request.command_canonical);
        Ok(EngineOutput {
            state_after_canonical: state_after,
            outcome_canonical: b"outcome:victory".to_vec(),
            replay_canonical: b"replay:frame-0".to_vec(),
            steps_used: 3,
        })
    }
}

fn vector_request() -> TransitionRequest {
    TransitionRequest::new(
        "first_contact_v1",
        "content_2026_08_27",
        "vector-0001",
        b"state-v1".to_vec(),
        b"command-v1".to_vec(),
        ResourceBudget {
            max_steps: 100,
            max_output_bytes: 4096,
            max_replay_bytes: 4096,
        },
    )
}

fn main() {
    let request = vector_request();
    let receipt = execute_transition_verified(&VectorEngine, &request)
        .expect("built-in vector request must satisfy the contract envelope");
    let mode = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    match mode.as_str() {
        "request" => print_bytes(&request.canonical_bytes()),
        "result" => print_bytes(&receipt.canonical_bytes()),
        "all" => {
            println!("[request]");
            print_bytes(&request.canonical_bytes());
            println!("[result]");
            print_bytes(&receipt.canonical_bytes());
        }
        _ => {
            eprintln!("usage: trnm-world-rules-vector [all|request|result]");
            std::process::exit(2);
        }
    }
}

fn print_bytes(bytes: &[u8]) {
    let text = std::str::from_utf8(bytes).expect("canonical vectors are UTF-8 lines");
    print!("{text}");
}
