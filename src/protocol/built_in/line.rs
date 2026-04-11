//! Line-based protocol implementation

use crate::error::Result;
use crate::protocol::Protocol;

/// Line-based protocol handler
#[derive(Clone)]
pub struct LineProtocol {
    separator: Vec<u8>,
}

impl LineProtocol {
    /// Create a new line-based protocol
    pub fn new() -> Self {
        Self {
            separator: vec![b'\n'],
        }
    }

    /// Set separator
    pub fn with_separator(mut self, sep: Vec<u8>) -> Self {
        self.separator = sep;
        self
    }
}

impl Default for LineProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Protocol for LineProtocol {
    fn name(&self) -> &str {
        "line"
    }

    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        // Just return the data as-is
        Ok(data.to_vec())
    }

    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        // Append separator if not already present
        if data.ends_with(&self.separator) {
            Ok(data.to_vec())
        } else {
            let mut result = data.to_vec();
            result.extend_from_slice(&self.separator);
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_adds_newline() {
        let mut proto = LineProtocol::new();
        let result = proto.encode(b"hello").unwrap();
        assert_eq!(result, b"hello\n");
    }

    #[test]
    fn test_encode_skips_existing_newline() {
        let mut proto = LineProtocol::new();
        let result = proto.encode(b"hello\n").unwrap();
        assert_eq!(result, b"hello\n");
    }

    #[test]
    fn test_encode_empty_data() {
        let mut proto = LineProtocol::new();
        let result = proto.encode(b"").unwrap();
        assert_eq!(result, b"\n");
    }

    #[test]
    fn test_encode_only_newline() {
        let mut proto = LineProtocol::new();
        let result = proto.encode(b"\n").unwrap();
        assert_eq!(result, b"\n");
    }

    #[test]
    fn test_parse_returns_data_as_is() {
        let mut proto = LineProtocol::new();
        let result = proto.parse(b"hello\n").unwrap();
        assert_eq!(result, b"hello\n");
    }

    #[test]
    fn test_custom_separator() {
        let mut proto = LineProtocol::new().with_separator(vec![b'\r', b'\n']);
        let encoded = proto.encode(b"cmd").unwrap();
        assert_eq!(encoded, b"cmd\r\n");
        // Should not duplicate
        let encoded2 = proto.encode(b"cmd\r\n").unwrap();
        assert_eq!(encoded2, b"cmd\r\n");
    }

    #[test]
    fn test_name() {
        let proto = LineProtocol::new();
        assert_eq!(proto.name(), "line");
    }
}
