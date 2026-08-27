const MAX_NESTING_DEPTH: usize = 128;

pub(super) fn validate(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() < 2 || !matches!(bytes.first(), Some(b'{') | Some(b'[')) {
        return false;
    }
    let mut parser = Parser { bytes, index: 0 };
    parser.parse_value(0) && parser.index == bytes.len()
}

struct Parser<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl Parser<'_> {
    fn parse_value(&mut self, depth: usize) -> bool {
        if depth > MAX_NESTING_DEPTH {
            return false;
        }
        match self.peek() {
            Some(b'{') => self.parse_object(depth),
            Some(b'[') => self.parse_array(depth),
            Some(b'"') => self.parse_string().is_some(),
            Some(b't') => self.consume_literal(b"true"),
            Some(b'f') => self.consume_literal(b"false"),
            Some(b'n') => self.consume_literal(b"null"),
            Some(b'-' | b'0'..=b'9') => self.parse_integer(),
            _ => false,
        }
    }

    fn parse_object(&mut self, depth: usize) -> bool {
        if !self.consume(b'{') {
            return false;
        }
        if self.consume(b'}') {
            return true;
        }

        let mut previous_key: Option<String> = None;
        loop {
            let Some(key) = self.parse_string() else {
                return false;
            };
            if previous_key
                .as_ref()
                .is_some_and(|previous| previous.as_str() >= key.as_str())
            {
                return false;
            }
            previous_key = Some(key);
            if !self.consume(b':') || !self.parse_value(depth + 1) {
                return false;
            }
            if self.consume(b'}') {
                return true;
            }
            if !self.consume(b',') {
                return false;
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> bool {
        if !self.consume(b'[') {
            return false;
        }
        if self.consume(b']') {
            return true;
        }
        loop {
            if !self.parse_value(depth + 1) {
                return false;
            }
            if self.consume(b']') {
                return true;
            }
            if !self.consume(b',') {
                return false;
            }
        }
    }

    fn parse_string(&mut self) -> Option<String> {
        if !self.consume(b'"') {
            return None;
        }
        let mut decoded = String::new();
        loop {
            let byte = self.peek()?;
            match byte {
                b'"' => {
                    self.index += 1;
                    return Some(decoded);
                }
                b'\\' => {
                    self.index += 1;
                    self.parse_escape(&mut decoded)?;
                }
                0x00..=0x1f => return None,
                0x20..=0x7f => {
                    self.index += 1;
                    decoded.push(char::from(byte));
                }
                _ => {
                    let remaining = std::str::from_utf8(&self.bytes[self.index..]).ok()?;
                    let character = remaining.chars().next()?;
                    self.index += character.len_utf8();
                    decoded.push(character);
                }
            }
        }
    }

    fn parse_escape(&mut self, decoded: &mut String) -> Option<()> {
        let escape = self.peek()?;
        self.index += 1;
        match escape {
            b'"' => decoded.push('"'),
            b'\\' => decoded.push('\\'),
            b'b' => decoded.push('\u{0008}'),
            b'f' => decoded.push('\u{000c}'),
            b'n' => decoded.push('\n'),
            b'r' => decoded.push('\r'),
            b't' => decoded.push('\t'),
            b'u' => {
                let codepoint = self.parse_lower_hex_quad()?;
                if codepoint > 0x1f || matches!(codepoint, 0x08 | 0x09 | 0x0a | 0x0c | 0x0d) {
                    return None;
                }
                decoded.push(char::from_u32(u32::from(codepoint))?);
            }
            _ => return None,
        }
        Some(())
    }

    fn parse_lower_hex_quad(&mut self) -> Option<u16> {
        let mut value = 0u16;
        for _ in 0..4 {
            let digit = match self.peek()? {
                b'0'..=b'9' => u16::from(self.bytes[self.index] - b'0'),
                b'a'..=b'f' => u16::from(self.bytes[self.index] - b'a' + 10),
                _ => return None,
            };
            self.index += 1;
            value = value.checked_mul(16)?.checked_add(digit)?;
        }
        Some(value)
    }

    fn parse_integer(&mut self) -> bool {
        let start = self.index;
        let negative = self.consume(b'-');
        match self.peek() {
            Some(b'0') => {
                self.index += 1;
                if negative || matches!(self.peek(), Some(b'0'..=b'9')) {
                    return false;
                }
            }
            Some(b'1'..=b'9') => {
                self.index += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.index += 1;
                }
            }
            _ => return false,
        }
        if matches!(self.peek(), Some(b'.' | b'e' | b'E')) {
            return false;
        }
        std::str::from_utf8(&self.bytes[start..self.index])
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .is_some()
    }

    fn consume_literal(&mut self, literal: &[u8]) -> bool {
        if self.bytes.get(self.index..self.index + literal.len()) != Some(literal) {
            return false;
        }
        self.index += literal.len();
        true
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }
        self.index += 1;
        true
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_sorted_minimal_integer_json() {
        assert!(validate(
            "{\"a\":[-9,0,17,true,false,null],\"b\":{\"c\":\"世界\"}}"
        ));
        assert!(validate("[]"));
        assert!(validate("{}"));
    }

    #[test]
    fn rejects_whitespace_and_incomplete_grammar() {
        assert!(!validate("{ \"a\":1}"));
        assert!(!validate("{\"a\":}"));
        assert!(!validate("[1,]"));
        assert!(!validate("{\"a\":1}junk"));
    }

    #[test]
    fn rejects_duplicate_or_unsorted_object_keys() {
        assert!(!validate("{\"a\":1,\"a\":2}"));
        assert!(!validate("{\"b\":1,\"a\":2}"));
        assert!(validate("{\"a\":1,\"b\":2}"));
    }

    #[test]
    fn rejects_floats_noncanonical_integers_and_overflow() {
        assert!(!validate("[1.0]"));
        assert!(!validate("[1e3]"));
        assert!(!validate("[-0]"));
        assert!(!validate("[01]"));
        assert!(!validate("[9223372036854775808]"));
    }

    #[test]
    fn requires_minimal_string_escapes() {
        assert!(validate("[\"\\n\\u0001\\\"\\\\\"]"));
        assert!(!validate("[\"\\u000a\"]"));
        assert!(!validate("[\"\\/\"]"));
        assert!(!validate("[\"\\u4e16\"]"));
    }

    #[test]
    fn rejects_excessive_nesting() {
        let payload = format!("{}0{}", "[".repeat(130), "]".repeat(130));
        assert!(!validate(&payload));
    }
}
