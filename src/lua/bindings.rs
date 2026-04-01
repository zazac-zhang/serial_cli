//! Lua API bindings
//!
//! This module provides the Rust API bindings for Lua scripts.

use crate::error::{Result, SerialError};
use mlua::{Lua, Value, Function};

/// Lua API bindings
pub struct LuaBindings {
    lua: Lua,
}

impl LuaBindings {
    /// Create new Lua bindings
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// Register logging API
    pub fn register_log_api(&self) -> Result<()> {
        let globals = self.lua.globals();

        // log.info
        let info = self.lua.create_function(|_, msg: String| {
            println!("[INFO] {}", msg);
            Ok(())
        })?;
        globals.set("log_info", info)?;

        // log.debug
        let debug = self.lua.create_function(|_, msg: String| {
            println!("[DEBUG] {}", msg);
            Ok(())
        })?;
        globals.set("log_debug", debug)?;

        // log.warn
        let warn = self.lua.create_function(|_, msg: String| {
            println!("[WARN] {}", msg);
            Ok(())
        })?;
        globals.set("log_warn", warn)?;

        // log.error
        let error = self.lua.create_function(|_, msg: String| {
            eprintln!("[ERROR] {}", msg);
            Ok(())
        })?;
        globals.set("log_error", error)?;

        Ok(())
    }

    /// Register utility APIs
    pub fn register_utility_apis(&self) -> Result<()> {
        let globals = self.lua.globals();

        // json.encode (simple version for basic types)
        let encode = self.lua.create_function(|_, value: Value| {
            // For now, just return a string representation
            Ok(format!("{:?}", value))
        })?;
        globals.set("json_encode", encode)?;

        // json.decode (not implemented - returns nil)
        let decode = self.lua.create_function(|_, _: String| {
            Ok(Value::Nil)
        })?;
        globals.set("json_decode", decode)?;

        // sleep
        let sleep = self.lua.create_function(|_, ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        })?;
        globals.set("sleep_ms", sleep)?;

        Ok(())
    }

    /// Register all APIs
    pub fn register_all_apis(&self) -> Result<()> {
        self.register_log_api()?;
        self.register_utility_apis()?;
        Ok(())
    }

    /// Execute a Lua script
    pub fn execute_script(&self, script: &str) -> Result<()> {
        self.lua.load(script)
            .exec()
            .map_err(|e| SerialError::Lua(e))
    }

    /// Execute a Lua function (simplified - returns success/failure)
    pub fn execute_function(&self, func_name: &str, args: Vec<Value>) -> Result<()> {
        let globals = self.lua.globals();
        let func: Function = globals.get(func_name)
            .map_err(|e| SerialError::Lua(e))?;

        func.call(args)
            .map(|_: Value| ())
            .map_err(|e| SerialError::Lua(e))
    }

    /// Get a global value (simplified)
    pub fn get_global(&self, name: &str) -> Result<Value<'_>> {
        let globals = self.lua.globals();
        globals.get(name)
            .map_err(|e| SerialError::Lua(e))
    }

    /// Set a global value (simplified)
    pub fn set_global(&self, name: &str, value: Value) -> Result<()> {
        let globals = self.lua.globals();
        globals.set(name, value)
            .map_err(|e| SerialError::Lua(e))
    }
}

impl Default for LuaBindings {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_creation() {
        let bindings = LuaBindings::new().unwrap();
        assert!(bindings.register_all_apis().is_ok());
    }

    #[test]
    fn test_simple_script() {
        let bindings = LuaBindings::new().unwrap();

        let script = r#"
            local x = 10
            local y = 20
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_script_with_print() {
        let bindings = LuaBindings::new().unwrap();

        let script = r#"
            print("Hello from Lua!")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_log_api() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_log_api().unwrap();

        let script = r#"
            log_info("Test info message")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_utility_apis() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_utility_apis().unwrap();

        let script = r#"
            local json_str = json_encode({test = "value"})
            print(json_str)
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }
}
