//! Protocol engine module
//!
//! This module provides protocol handling and parsing.

pub mod built_in;
pub mod lua_ext;
pub mod loader;
pub mod manager;
pub mod registry;
pub mod validator;
pub mod watcher;

pub use built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol};
pub use registry::{ProtocolFactory, ProtocolInfo, ProtocolRegistry};
pub use validator::{ProtocolValidator, ValidationResult};
pub use loader::{ProtocolLoader, LoadedProtocol};
pub use watcher::ProtocolWatcher;
pub use manager::{ProtocolManager, CustomProtocol};

// Export Lua protocol for external use
pub use lua_ext::{LuaProtocol, create_lua_protocol};

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
