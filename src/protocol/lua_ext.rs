//! Lua protocol extension interface
//!
//! This module allows users to define custom protocols in Lua.

use crate::error::{Result, SerialError, ProtocolError};
use crate::protocol::{Protocol, ProtocolStats};

/// Custom protocol defined in Lua (simplified version)
///
/// Note: This is a simplified implementation. Full Lua callback execution
/// will be implemented in Phase 3 (Lua Integration).
#[derive(Clone)]
pub struct LuaProtocol {
    name: String,
    script: Option<String>,
    stats: ProtocolStats,
}

impl LuaProtocol {
    /// Create a new Lua protocol from a Lua script
    pub fn from_script(name: String, script: &str) -> Result<Self> {
        // Validate the script syntax
        // TODO: In Phase 3, we'll actually execute the script
        Ok(Self {
            name,
            script: Some(script.to_string()),
            stats: ProtocolStats::default(),
        })
    }

    /// Create a new empty Lua protocol
    pub fn new(name: String) -> Result<Self> {
        Ok(Self {
            name,
            script: None,
            stats: ProtocolStats::default(),
        })
    }

    /// Get the script content
    pub fn script(&self) -> Option<&String> {
        self.script.as_ref()
    }
}

impl Protocol for LuaProtocol {
    fn name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.stats.frames_parsed += 1;

        // For now, just return the data as-is
        // TODO: In Phase 3, execute Lua callback
        Ok(data.to_vec())
    }

    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.stats.frames_encoded += 1;

        // For now, just return the data as-is
        // TODO: In Phase 3, execute Lua callback
        Ok(data.to_vec())
    }

    fn reset(&mut self) -> Result<()> {
        // TODO: In Phase 3, execute Lua reset callback
        Ok(())
    }

    fn stats(&self) -> ProtocolStats {
        self.stats.clone()
    }
}

/// Helper to create a Lua protocol from a string
pub fn create_lua_protocol(name: String, script: &str) -> Result<Box<dyn Protocol>> {
    let protocol = LuaProtocol::from_script(name, script)?;
    Ok(Box::new(protocol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_protocol_basic() {
        let protocol = LuaProtocol::new("test".to_string()).unwrap();
        assert_eq!(protocol.name(), "test");
        assert!(protocol.script().is_none());

        // Test default behavior (passthrough)
        let mut protocol = protocol;
        let data = vec![0x01, 0x02, 0x03];
        let encoded = protocol.encode(&data).unwrap();
        assert_eq!(encoded, data);

        let parsed = protocol.parse(&data).unwrap();
        assert_eq!(parsed, data);
    }

    #[test]
    fn test_lua_protocol_clone() {
        let protocol1 = LuaProtocol::new("clone_test".to_string()).unwrap();
        let protocol2 = protocol1.clone();
        assert_eq!(protocol1.name(), protocol2.name());
    }

    #[test]
    fn test_lua_protocol_from_script() {
        let script = r#"
            function on_frame(data)
                return data
            end

            function on_encode(data)
                return data
            end
        "#;

        let protocol = LuaProtocol::from_script("custom".to_string(), script).unwrap();
        assert_eq!(protocol.name(), "custom");
        assert!(protocol.script().is_some());
        assert_eq!(protocol.script().unwrap(), script);
    }

    #[test]
    fn test_lua_protocol_stats() {
        let mut protocol = LuaProtocol::new("stats_test".to_string()).unwrap();
        let data = vec![0x01, 0x02];

        protocol.encode(&data).unwrap();
        protocol.parse(&data).unwrap();

        let stats = protocol.stats();
        assert_eq!(stats.frames_encoded, 1);
        assert_eq!(stats.frames_parsed, 1);
    }
}
