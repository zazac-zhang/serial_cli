//! Lua API bindings
//!
//! This module provides the Rust API bindings for Lua scripts.

use crate::error::{Result, SerialError};
use crate::serial_core::PortManager;
use mlua::{Function, Lua, Value};
use std::sync::Arc;
use std::cell::RefCell;
use tokio::sync::Mutex;

/// Lua API bindings
pub struct LuaBindings {
    lua: Lua,
    port_manager: Option<Arc<Mutex<PortManager>>>,
    runtime: RefCell<Option<Arc<tokio::runtime::Runtime>>>,
}

impl LuaBindings {
    /// Create new Lua bindings
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self {
            lua,
            port_manager: None,
            runtime: RefCell::new(None),
        })
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
        let decode = self.lua.create_function(|_, _: String| Ok(Value::Nil))?;
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
        self.lua.load(script).exec().map_err(SerialError::Lua)
    }

    /// Execute a Lua function (simplified - returns success/failure)
    pub fn execute_function(&self, func_name: &str, args: Vec<Value>) -> Result<()> {
        let globals = self.lua.globals();
        let func: Function = globals.get(func_name).map_err(SerialError::Lua)?;

        func.call(args).map(|_: Value| ()).map_err(SerialError::Lua)
    }

    /// Get a global value (simplified)
    pub fn get_global(&self, name: &str) -> Result<Value<'_>> {
        let globals = self.lua.globals();
        globals.get(name).map_err(SerialError::Lua)
    }

    /// Set a global value (simplified)
    pub fn set_global(&self, name: &str, value: Value) -> Result<()> {
        let globals = self.lua.globals();
        globals.set(name, value).map_err(SerialError::Lua)
    }

    /// Set the port manager
    pub fn set_port_manager(&mut self, pm: Arc<Mutex<PortManager>>) {
        self.port_manager = Some(pm);
    }

    /// Ensure runtime is initialized
    fn ensure_runtime(&self) -> Result<()> {
        if self.runtime.borrow().is_none() {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| SerialError::Config(format!("Failed to create runtime: {}", e)))?;
            *self.runtime.borrow_mut() = Some(Arc::new(rt));
        }
        Ok(())
    }

    /// Register serial_open API
    pub fn register_serial_open(&self) -> Result<()> {
        self.ensure_runtime()?;

        let port_manager = self.port_manager.clone()
            .ok_or_else(|| SerialError::Config("PortManager not initialized".to_string()))?;

        let runtime = self.runtime.borrow()
            .as_ref()
            .ok_or_else(|| SerialError::Config("Runtime not initialized".to_string()))?
            .clone();

        let open = self.lua.create_function(move |_, (port_name, baudrate): (String, u32)| {
            let pm_guard = runtime.block_on(port_manager.lock());
            let config = crate::serial_core::SerialConfig {
                baudrate,
                ..Default::default()
            };

            let port_id = runtime.block_on(pm_guard.open_port(&port_name, config))
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(port_id)
        })?;

        self.lua.globals().set("serial_open", open)?;
        Ok(())
    }

    /// Register serial_close API
    pub fn register_serial_close(&self) -> Result<()> {
        self.ensure_runtime()?;

        let port_manager = self.port_manager.clone()
            .ok_or_else(|| SerialError::Config("PortManager not initialized".to_string()))?;

        let runtime = self.runtime.borrow()
            .as_ref()
            .ok_or_else(|| SerialError::Config("Runtime not initialized".to_string()))?
            .clone();

        let close = self.lua.create_function(move |_, port_id: String| {
            let pm_guard = runtime.block_on(port_manager.lock());

            runtime.block_on(pm_guard.close_port(&port_id))
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(true)
        })?;

        self.lua.globals().set("serial_close", close)?;
        Ok(())
    }

    /// Get the Lua instance
    pub fn lua(&self) -> &Lua {
        &self.lua
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

    #[test]
    fn test_serial_open_lua() {
        let mut bindings = LuaBindings::new().unwrap();
        let pm = Arc::new(Mutex::new(PortManager::new()));
        bindings.set_port_manager(pm);
        bindings.register_serial_open().unwrap();

        let script = r#"
            local ok, result = pcall(serial_open, "/dev/ttyUSB0", 115200)
            -- ok should be false (port doesn't exist)
            assert(ok == false, "Expected ok to be false but got " .. tostring(ok))

            -- result should be an error (can be string, userdata, or other type)
            -- The important thing is that pcall caught the error
            assert(result ~= nil, "Expected result to not be nil")

            -- Convert result to string to verify it's an error
            local result_str = tostring(result)
            assert(type(result_str) == "string", "Expected result_str to be string")
            assert(string.find(result_str, "Serial") ~= nil or
                   string.find(result_str, "not found") ~= nil,
                   "Expected error message to contain 'Serial' or 'not found', got: " .. result_str)
        "#;

        bindings.execute_script(script).unwrap();
    }

    #[test]
    fn test_serial_close_lua() {
        let mut bindings = LuaBindings::new().unwrap();
        let pm = Arc::new(Mutex::new(PortManager::new()));
        bindings.set_port_manager(pm);
        bindings.register_serial_close().unwrap();

        let script = r#"
            local ok, result = pcall(serial_close, "nonexistent-port")
            -- ok should be false (port doesn't exist)
            assert(ok == false, "Expected ok to be false but got " .. tostring(ok))

            -- result should be an error
            assert(result ~= nil, "Expected result to not be nil")

            -- Convert result to string to verify it's an error
            local result_str = tostring(result)
            assert(type(result_str) == "string", "Expected result_str to be string")
        "#;

        bindings.execute_script(script).unwrap();
    }
}
