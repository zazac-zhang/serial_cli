//! Protocol engine module
//!
//! This module provides protocol handling and parsing.

pub mod registry;
pub mod built_in;
pub mod lua_ext;

pub use registry::{ProtocolRegistry, ProtocolFactory, ProtocolInfo};
pub use built_in::{ModbusProtocol, AtCommandProtocol, LineProtocol};

/// Protocol trait for serial communication protocols
pub trait Protocol: Send + Sync {
    /// Get protocol name
    fn name(&self) -> &str;

    /// Parse incoming data
    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>>;

    /// Encode outgoing data
    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>>;

    /// Reset protocol state
    fn reset(&mut self) -> Result<()> {
        Ok(())
    }

    /// Check if protocol has data ready
    fn has_data(&self) -> bool {
        false
    }

    /// Get protocol statistics
    fn stats(&self) -> ProtocolStats {
        ProtocolStats::default()
    }
}

/// Protocol statistics
#[derive(Debug, Clone, Default)]
pub struct ProtocolStats {
    pub frames_parsed: usize,
    pub frames_encoded: usize,
    pub errors: usize,
}

// Re-export for use in other modules
pub use crate::error::{Result, SerialError};
