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
    pub fn execute_with_args(&self, script: &str, args: Vec<String>) -> Result<()> {
        // Create the 'arg' table in Lua
        let lua = self.bindings.lua();
        let globals = lua.globals();

        // Create arg table
        let arg_table = lua.create_table()?;

        // Set arg[0] to script name (if available) or empty string
        arg_table.set(0, "script")?;

        // Set arg[1], arg[2], ... to the provided arguments
        for (i, arg) in args.iter().enumerate() {
            arg_table.set(i + 1, arg.clone())?;
        }

        // Set arg.n to the number of arguments
        arg_table.set("n", args.len())?;

        // Set the arg table as a global
        globals.set("arg", arg_table)?;

        // Also set individual global variables for convenience
        for (i, arg) in args.iter().enumerate() {
            let var_name = format!("arg{}", i + 1);
            globals.set(var_name, arg.clone())?;
        }

        // Execute the script
        self.bindings.execute_script(script)?;

        Ok(())
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
