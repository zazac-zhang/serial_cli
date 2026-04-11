//! AT Command protocol implementation

use crate::error::{ProtocolError, Result, SerialError};
use crate::protocol::Protocol;

/// AT Command protocol handler
#[derive(Clone)]
pub struct AtCommandProtocol {
    timeout_ms: u64,
    termination: String,
}

impl AtCommandProtocol {
    /// Create a new AT Command protocol
    pub fn new() -> Self {
        Self {
            timeout_ms: 1000,
            termination: "\r\n".to_string(),
        }
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Set termination string
    pub fn with_termination(mut self, term: String) -> Self {
        self.termination = term;
        self
    }
}

impl Default for AtCommandProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Protocol for AtCommandProtocol {
    fn name(&self) -> &str {
        "at_command"
    }

    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        // Convert to string and check for common responses
        let response = String::from_utf8_lossy(data);

        // Check for OK response
        if response.contains("OK") {
            return Ok(data.to_vec());
        }

        // Check for ERROR response
        if response.contains("ERROR") {
            return Err(SerialError::Protocol(ProtocolError::UnexpectedResponse(
                "AT command returned ERROR".to_string(),
            )));
        }

        // Return data as-is
        Ok(data.to_vec())
    }

    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        // Append termination
        let mut command = data.to_vec();
        command.extend_from_slice(self.termination.as_bytes());
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_adds_termination() {
        let mut proto = AtCommandProtocol::new();
        let result = proto.encode(b"ATZ").unwrap();
        assert_eq!(result, b"ATZ\r\n");
    }

    #[test]
    fn test_encode_with_custom_termination() {
        let mut proto = AtCommandProtocol::new().with_termination("\n".to_string());
        let result = proto.encode(b"ATZ").unwrap();
        assert_eq!(result, b"ATZ\n");
    }

    #[test]
    fn test_encode_appends_regardless_of_existing_termination() {
        let mut proto = AtCommandProtocol::new();
        // Unlike line protocol, AT always appends termination
        let result = proto.encode(b"ATZ\r\n").unwrap();
        assert_eq!(result, b"ATZ\r\n\r\n");
    }

    #[test]
    fn test_parse_ok_response() {
        let mut proto = AtCommandProtocol::new();
        let result = proto.parse(b"OK").unwrap();
        assert_eq!(result, b"OK");
    }

    #[test]
    fn test_parse_ok_in_mixed_response() {
        let mut proto = AtCommandProtocol::new();
        let result = proto.parse(b"+CSQ: 20,99\r\nOK").unwrap();
        assert_eq!(result, b"+CSQ: 20,99\r\nOK");
    }

    #[test]
    fn test_parse_error_response() {
        let mut proto = AtCommandProtocol::new();
        let result = proto.parse(b"ERROR");
        assert!(result.is_err());
        match result.unwrap_err() {
            SerialError::Protocol(ProtocolError::UnexpectedResponse(msg)) => {
                assert!(msg.contains("ERROR"));
            }
            other => panic!("Expected UnexpectedResponse, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_unknown_response() {
        let mut proto = AtCommandProtocol::new();
        // Non-OK/non-ERROR responses pass through
        let result = proto.parse(b"+CME ERROR: 12");
        // Contains "ERROR" so should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_data() {
        let mut proto = AtCommandProtocol::new();
        let result = proto.parse(b"").unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn test_name() {
        let proto = AtCommandProtocol::new();
        assert_eq!(proto.name(), "at_command");
    }

    #[test]
    fn test_with_timeout() {
        let proto = AtCommandProtocol::new().with_timeout(5000);
        assert_eq!(proto.timeout_ms, 5000);
    }
}
