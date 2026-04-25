//! Protocol engine module
//!
//! This module provides protocol handling and parsing.

pub mod built_in;
pub mod loader;
pub mod lua_ext;
pub mod manager;
pub mod registration;
pub mod registry;
pub mod validator;
pub mod watcher;

pub use built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol};
pub use loader::{LoadedProtocol, ProtocolLoader};
pub use manager::{CustomProtocol, ProtocolManager};
pub use registration::register_all_built_in;
pub use registry::{ProtocolFactory, ProtocolInfo, ProtocolRegistry};
pub use validator::{ProtocolValidator, ValidationResult};
pub use watcher::ProtocolWatcher;

// Export Lua protocol for external use
pub use lua_ext::{create_lua_protocol, LuaProtocol};

/// Core trait for serial communication protocol implementations.
///
/// All protocols (Modbus RTU, Modbus ASCII, AT commands, line-based, custom Lua)
/// implement this trait. The engine calls [`parse`](Self::parse) on incoming
/// data and [`encode`](Self::encode) on outgoing data.
///
/// Implementations must be `Send + Sync` for use in async contexts.
pub trait Protocol: Send + Sync {
    /// Get the protocol's unique name (e.g., `"modbus_rtu"`).
    fn name(&self) -> &str;

    /// Parse incoming raw bytes into a protocol frame.
    ///
    /// Returns the parsed payload as a byte vector.
    ///
    /// # Errors
    ///
    /// Returns [`SerialError::Protocol`] if the data is malformed,
    /// a checksum fails, or the frame is incomplete.
    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>>;

    /// Encode outgoing data into a protocol frame (e.g., add headers, checksums).
    ///
    /// # Errors
    ///
    /// Returns [`SerialError::Protocol`] if encoding fails (should not
    /// normally occur for well-formed input).
    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>>;

    /// Reset internal parser state. The default implementation is a no-op.
    fn reset(&mut self) -> Result<()> {
        Ok(())
    }

    /// Check whether the protocol has a complete frame ready for consumption.
    fn has_data(&self) -> bool {
        false
    }

    /// Return cumulative parsing/encoding statistics.
    fn stats(&self) -> ProtocolStats {
        ProtocolStats::default()
    }
}

/// Counters tracking protocol parsing and encoding activity.
#[derive(Debug, Clone, Default)]
pub struct ProtocolStats {
    /// Number of frames successfully parsed.
    pub frames_parsed: usize,
    /// Number of frames successfully encoded.
    pub frames_encoded: usize,
    /// Number of parse or encode errors encountered.
    pub errors: usize,
}

// Re-export for use in other modules
pub use crate::error::{Result, SerialError};
