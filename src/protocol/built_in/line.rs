//! Line-based protocol implementation

use crate::protocol::{Protocol, ProtocolStats};
use crate::error::Result;

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
