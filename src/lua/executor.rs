//! Script execution engine

use crate::error::{Result, SerialError};
use crate::lua::bindings::LuaBindings;
use crate::serial_core::PortManager;
use std::fs;
use std::path::Path;

/// Script execution engine
pub struct ScriptEngine {
    pub bindings: LuaBindings,
    port_manager: PortManager,
}

impl ScriptEngine {
    /// Create a new script engine
    pub fn new() -> Result<Self> {
        Ok(Self {
            bindings: LuaBindings::new()?,
            port_manager: PortManager::new(),
        })
    }

    /// Execute a script from a string
    pub fn execute_string(&self, script: &str) -> Result<()> {
        self.bindings.execute_script(script)
    }

    /// Execute a script from a file
    pub fn execute_file(&self, path: &Path) -> Result<()> {
        let script = fs::read_to_string(path).map_err(SerialError::Io)?;

        self.bindings.execute_script(&script)
    }

    /// Execute a script with arguments
    pub fn execute_with_args(&self, script: &str, _args: Vec<String>) -> Result<()> {
        // TODO: Implement argument passing
        self.bindings.execute_script(script)
    }

    /// Get the port manager
    pub fn port_manager(&self) -> &PortManager {
        &self.port_manager
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = ScriptEngine::new().unwrap();
        assert!(engine.execute_string("print('test')").is_ok());
    }

    #[test]
    fn test_execute_math() {
        let engine = ScriptEngine::new().unwrap();
        let script = r#"
            local result = 2 + 2
            assert(result == 4, "Math failed")
        "#;
        assert!(engine.execute_string(script).is_ok());
    }
}
