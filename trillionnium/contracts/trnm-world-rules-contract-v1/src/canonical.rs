use crate::digest::hex_encode;

pub const REQUEST_DOMAIN: &str = "TRNM-WORLD-RULES-REQUEST/1";
pub const RESULT_DOMAIN: &str = "TRNM-WORLD-RULES-RESULT/1";

#[derive(Debug, Default)]
pub(crate) struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    pub(crate) fn with_domain(domain: &str) -> Self {
        let mut writer = Self::default();
        writer.line(domain);
        writer
    }

    pub(crate) fn field(&mut self, name: &str, value: &str) {
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes.push(b'=');
        self.bytes.extend_from_slice(value.as_bytes());
        self.bytes.push(b'\n');
    }

    pub(crate) fn u64_field(&mut self, name: &str, value: u64) {
        self.field(name, &value.to_string());
    }

    pub(crate) fn bytes_field(&mut self, name: &str, value: &[u8]) {
        self.field(name, &hex_encode(value));
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn line(&mut self, value: &str) {
        self.bytes.extend_from_slice(value.as_bytes());
        self.bytes.push(b'\n');
    }
}

pub(crate) fn safe_token(value: &str, maximum_length: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_length
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'@' | b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_writer_has_stable_order_and_newline_termination() {
        let mut writer = CanonicalWriter::with_domain("DOMAIN/1");
        writer.field("alpha", "one");
        writer.u64_field("count", 2);
        writer.bytes_field("payload", b"A");
        assert_eq!(
            writer.finish(),
            b"DOMAIN/1\nalpha=one\ncount=2\npayload=41\n"
        );
    }

    #[test]
    fn tokens_reject_whitespace_separators_and_controls() {
        assert!(safe_token("ruleset-v1.2:alpha", 64));
        assert!(!safe_token("", 64));
        assert!(!safe_token("contains space", 64));
        assert!(!safe_token("contains\nnewline", 64));
        assert!(!safe_token("contains=separator", 64));
    }
}
