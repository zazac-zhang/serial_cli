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

    /// Register serial_send API
    pub fn register_serial_send(&self) -> Result<()> {
        self.ensure_runtime()?;

        let port_manager = self.port_manager.clone()
            .ok_or_else(|| SerialError::Config("PortManager not initialized".to_string()))?;

        let runtime = self.runtime.borrow()
            .as_ref()
            .ok_or_else(|| SerialError::Config("Runtime not initialized".to_string()))?
            .clone();

        let send = self.lua.create_function(move |_, (port_id, data): (String, String)| {
            let pm_guard = runtime.block_on(port_manager.lock());

            let port_handle = runtime.block_on(pm_guard.get_port(&port_id))
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            let mut handle = runtime.block_on(port_handle.lock());
            let bytes = handle.write(data.as_bytes())
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(bytes)
        })?;

        self.lua.globals().set("serial_send", send)?;
        Ok(())
    }

    /// Register serial_recv API
    pub fn register_serial_recv(&self) -> Result<()> {
        self.ensure_runtime()?;

        let port_manager = self.port_manager.clone()
            .ok_or_else(|| SerialError::Config("PortManager not initialized".to_string()))?;

        let runtime = self.runtime.borrow()
            .as_ref()
            .ok_or_else(|| SerialError::Config("Runtime not initialized".to_string()))?
            .clone();

        let recv = self.lua.create_function(move |_, (port_id, timeout_ms): (String, u64)| {
            let pm_guard = runtime.block_on(port_manager.lock());

            let port_handle = runtime.block_on(pm_guard.get_port(&port_id))
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            // Clone runtime for the async block
            let rt_clone = runtime.clone();

            // Wrap synchronous read in async block for timeout
            let read_future = async move {
                // Move port_handle into the blocking task
                let read_result = tokio::task::spawn_blocking(move || {
                    let mut handle = rt_clone.block_on(port_handle.lock());
                    let mut buffer = vec![0u8; 4096];

                    let n = handle.read(&mut buffer)?;

                    buffer.truncate(n);
                    Ok(String::from_utf8_lossy(&buffer).to_string())
                }).await
                .map_err(|e| crate::error::SerialError::Serial(
                    crate::error::SerialPortError::IoError(e.to_string())
                ))?;

                read_result
            };

            // Apply timeout
            let result = tokio::time::timeout(
                std::time::Duration::from_millis(timeout_ms),
                read_future
            );

            let data = runtime.block_on(result)
                .map_err(|_| mlua::Error::RuntimeError("Timeout".to_string()))?
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(data)
        })?;

        self.lua.globals().set("serial_recv", recv)?;
        Ok(())
    }

    /// Register serial_list API
    pub fn register_serial_list(&self) -> Result<()> {
        self.ensure_runtime()?;

        let port_manager = self.port_manager.clone()
            .ok_or_else(|| SerialError::Config("PortManager not initialized".to_string()))?;

        let runtime = self.runtime.borrow()
            .as_ref()
            .ok_or_else(|| SerialError::Config("Runtime not initialized".to_string()))?
            .clone();

        let list = self.lua.create_function(move |lua, ()| {
            let pm_guard = runtime.block_on(port_manager.lock());

            let ports = pm_guard.list_ports()
                .map_err(|e: crate::error::SerialError| mlua::Error::RuntimeError(e.to_string()))?;

            // Convert to Lua table
            let result = lua.create_table()?;
            for (i, port) in ports.iter().enumerate() {
                let port_table = lua.create_table()?;
                port_table.set("port_name", port.port_name.clone())?;
                port_table.set("port_type", port.port_type.clone())?;
                result.set(i + 1, port_table)?;
            }

            Ok(result)
        })?;

        self.lua.globals().set("serial_list", list)?;
        Ok(())
    }

    /// Get the Lua instance
    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    /// Helper function to register built-in protocols
    async fn register_builtins(registry: &mut crate::protocol::ProtocolRegistry) {
        use crate::protocol::built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol};
        use crate::protocol::registry::SimpleProtocolFactory;

        registry.register(SimpleProtocolFactory::new(
            "line".to_string(),
            "Line-based protocol".to_string(),
            LineProtocol::new,
        )).await;

        registry.register(SimpleProtocolFactory::new(
            "at_command".to_string(),
            "AT Command protocol".to_string(),
            AtCommandProtocol::new,
        )).await;

        registry.register(SimpleProtocolFactory::new(
            "modbus_rtu".to_string(),
            "Modbus RTU protocol".to_string(),
            || ModbusProtocol::new(crate::protocol::built_in::modbus::ModbusMode::Rtu),
        )).await;

        registry.register(SimpleProtocolFactory::new(
            "modbus_ascii".to_string(),
            "Modbus ASCII protocol".to_string(),
            || ModbusProtocol::new(crate::protocol::built_in::modbus::ModbusMode::Ascii),
        )).await;
    }

    /// Register protocol_encode API
    pub fn register_protocol_encode(&self) -> Result<()> {
        let encode = self.lua.create_function(move |_, (protocol_name, data): (String, String)| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| mlua::Error::RuntimeError(format!("Failed to create runtime: {}", e)))?;
            let mut registry = ProtocolRegistry::new();

            rt.block_on(Self::register_builtins(&mut registry));

            let mut protocol = rt.block_on(registry.get_protocol(&protocol_name))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let encoded = protocol.encode(data.as_bytes())
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(String::from_utf8_lossy(&encoded).to_string())
        })?;

        self.lua.globals().set("protocol_encode", encode)?;
        Ok(())
    }

    /// Register protocol_decode API
    pub fn register_protocol_decode(&self) -> Result<()> {
        let decode = self.lua.create_function(move |_, (protocol_name, data): (String, String)| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| mlua::Error::RuntimeError(format!("Failed to create runtime: {}", e)))?;
            let mut registry = ProtocolRegistry::new();

            rt.block_on(Self::register_builtins(&mut registry));

            let mut protocol = rt.block_on(registry.get_protocol(&protocol_name))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let decoded = protocol.parse(data.as_bytes())
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(String::from_utf8_lossy(&decoded).to_string())
        })?;

        self.lua.globals().set("protocol_decode", decode)?;
        Ok(())
    }

    /// Register protocol_list API
    pub fn register_protocol_list(&self) -> Result<()> {
        let list = self.lua.create_function(|lua, ()| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| mlua::Error::RuntimeError(format!("Failed to create runtime: {}", e)))?;
            let mut registry = ProtocolRegistry::new();

            rt.block_on(Self::register_builtins(&mut registry));

            let protocols = rt.block_on(registry.list_protocols());

            let result = lua.create_table()?;
            for (i, protocol) in protocols.iter().enumerate() {
                let proto_table = lua.create_table()?;
                proto_table.set("name", protocol.name.clone())?;
                proto_table.set("description", protocol.description.clone())?;
                result.set(i + 1, proto_table)?;
            }

            Ok(result)
        })?;

        self.lua.globals().set("protocol_list", list)?;
        Ok(())
    }

    /// Register protocol_info API
    pub fn register_protocol_info(&self) -> Result<()> {
        let info = self.lua.create_function(|lua, protocol_name: String| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| mlua::Error::RuntimeError(format!("Failed to create runtime: {}", e)))?;
            let mut registry = ProtocolRegistry::new();

            rt.block_on(Self::register_builtins(&mut registry));

            let protocols = rt.block_on(registry.list_protocols());
            let protocol = protocols.iter()
                .find(|p| p.name == protocol_name)
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Protocol not found: {}", protocol_name)))?;

            let result = lua.create_table()?;
            result.set("name", protocol.name.clone())?;
            result.set("description", protocol.description.clone())?;
            Ok(result)
        })?;

        self.lua.globals().set("protocol_info", info)?;
        Ok(())
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

    #[test]
    fn test_serial_send_lua() {
        let mut bindings = LuaBindings::new().unwrap();
        let pm = Arc::new(Mutex::new(PortManager::new()));
        bindings.set_port_manager(pm);
        bindings.register_serial_send().unwrap();

        let script = r#"
            local ok, result = pcall(serial_send, "test-port", "Hello")
            -- Will fail (port doesn't exist) but tests the API
            assert(ok == false, "Expected ok to be false but got " .. tostring(ok))
            assert(result ~= nil, "Expected result to not be nil")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_serial_recv_lua() {
        let mut bindings = LuaBindings::new().unwrap();
        let pm = Arc::new(Mutex::new(PortManager::new()));
        bindings.set_port_manager(pm);
        bindings.register_serial_recv().unwrap();

        let script = r#"
            local ok, result = pcall(serial_recv, "test-port", 1000)
            -- Will fail but tests the API
            assert(ok == false, "Expected ok to be false but got " .. tostring(ok))
            assert(result ~= nil, "Expected result to not be nil")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_serial_list_lua() {
        let mut bindings = LuaBindings::new().unwrap();
        let pm = Arc::new(Mutex::new(PortManager::new()));
        bindings.set_port_manager(pm);
        bindings.register_serial_list().unwrap();

        let script = r#"
            local ports = serial_list()
            assert(type(ports) == "table", "Expected ports to be a table")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_encode_lua() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_encode().unwrap();

        let script = r#"
            -- Test line protocol
            local encoded = protocol_encode("line", "Hello")
            assert(type(encoded) == "string", "Expected string output")
            assert(string.sub(encoded, -1) == "\n", "Expected newline at end")

            -- Test at_command protocol
            local encoded_at = protocol_encode("at_command", "ATZ")
            assert(type(encoded_at) == "string", "Expected string output for AT command")

            -- Test modbus_rtu protocol
            local encoded_modbus = protocol_encode("modbus_rtu", "\x01\x03\x00\x00\x00\x01")
            assert(type(encoded_modbus) == "string", "Expected string output for Modbus")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_encode_invalid_protocol() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_encode().unwrap();

        let script = r#"
            local ok, err = pcall(protocol_encode, "invalid_protocol", "test")
            assert(ok == false, "Expected error for invalid protocol")
            assert(err ~= nil, "Expected error message")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_decode_lua() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_decode().unwrap();

        let script = r#"
            local decoded = protocol_decode("line", "Hello\n")
            assert(type(decoded) == "string")
            assert(decoded == "Hello\n")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_decode_invalid_protocol() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_decode().unwrap();

        let script = r#"
            local ok, err = pcall(protocol_decode, "invalid_protocol", "test\n")
            assert(ok == false, "Expected error for invalid protocol")
            assert(err ~= nil, "Expected error message")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_list_lua() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_list().unwrap();

        let script = r#"
            local protocols = protocol_list()
            assert(type(protocols) == "table", "Expected protocols to be a table")
            -- Should have at least line, at_command, modbus_rtu, modbus_ascii
            assert(#protocols >= 4, "Expected at least 4 protocols, got " .. #protocols)
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }

    #[test]
    fn test_protocol_info_lua() {
        let bindings = LuaBindings::new().unwrap();
        bindings.register_protocol_info().unwrap();

        let script = r#"
            local info = protocol_info("line")
            assert(type(info) == "table")
            assert(info.name == "line")
            assert(type(info.description) == "string")
        "#;

        assert!(bindings.execute_script(script).is_ok());
    }
}
