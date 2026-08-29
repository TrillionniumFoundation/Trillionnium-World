// The reviewed CEX/signer transport body is generated from src/cex.rs.in by
// build.rs. The transform is fail-closed, removes blocking transport support,
// bounds error bodies, and classifies malformed success responses as ambiguous
// retryable outcomes so the next attempt performs receipt lookup first.
include!(concat!(env!("OUT_DIR"), "/trnm_cex_generated.rs"));
