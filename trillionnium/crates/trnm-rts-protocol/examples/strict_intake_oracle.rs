//! Bounded JSON-lines test oracle. Not a server, authentication or release gate.
use std::io::{self, BufRead, Read, Write};

use serde_json::json;
use sha2::{Digest, Sha256};
use trnm_rts_protocol::strict;

const MAX_LINE: usize = (strict::MAX_INPUT_BYTES + 1) * 2 + 1;
const MAX_CASES: usize = 4096;

fn nibble(byte: u8) -> Result<u8, &'static str> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err("invalid oracle hex"),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = io::stdin().lock();
    let mut output = io::BufWriter::new(io::stdout().lock());
    let mut sequence = 0;
    loop {
        // take() bounds allocation even for a peer that never sends a newline.
        let mut line = Vec::new();
        let read = input
            .by_ref()
            .take((MAX_LINE + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if read == 0 {
            break;
        }
        if line.len() > MAX_LINE || line.last() != Some(&b'\n') || sequence >= MAX_CASES {
            return Err("oracle framing budget exceeded".into());
        }
        line.pop();
        if line.len() % 2 != 0 {
            return Err("odd oracle hex length".into());
        }
        let mut raw = Vec::with_capacity(line.len() / 2);
        for pair in line.chunks_exact(2) {
            raw.push((nibble(pair[0])? << 4) | nibble(pair[1])?);
        }
        let (result, order_sha256) = match strict::decode(&raw) {
            Ok(order) => {
                let bytes = serde_json::to_vec(order.as_order())?;
                let hash = format!("{:x}", Sha256::digest(&bytes));
                ("accepted", Some(hash))
            }
            Err(error) => (error.code(), None),
        };
        serde_json::to_writer(
            &mut output,
            &json!({"schema":"trnm_rts_intake_oracle_v1", "sequence":sequence,
                "result":result, "order_sha256":order_sha256}),
        )?;
        output.write_all(b"\n")?;
        sequence += 1;
    }
    if sequence == 0 {
        return Err("oracle received no cases".into());
    }
    output.flush()?;
    Ok(())
}

fn main() {
    if run().is_err() {
        // Never reflect malformed wire bytes, file contents, or parser diagnostics.
        eprintln!("strict intake oracle failed");
        std::process::exit(1);
    }
}
