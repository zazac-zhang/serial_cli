//! AT Command protocol implementation

use crate::protocol::{Protocol, ProtocolStats};
use crate::error::{Result, SerialError, ProtocolError};

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
