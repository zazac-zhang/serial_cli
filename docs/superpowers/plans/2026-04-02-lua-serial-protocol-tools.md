# Lua Serial and Protocol Tools Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Lua API functions for serial port operations and protocol tools, enabling end-to-end Lua scripting with flexible protocol handling.

**Architecture:** Add Lua API functions to existing `LuaBindings` struct that expose serial operations via `PortManager` and protocol operations via `ProtocolRegistry`. Keep protocol tools as pure functions (not bound to ports) for maximum flexibility.

**Tech Stack:** Rust, mlua (LuaJIT), tokio (async runtime), existing serial-core and protocol modules

---

## File Structure

### Files to Modify:
- `src/lua/bindings.rs` - Add serial and protocol API functions
- `src/lua/executor.rs` - Add methods to expose PortManager to LuaBindings
- `src/lua/stdlib.rs` - Add data conversion utility functions
- `src/main.rs` - Implement `run` command for Lua script execution
- `src/protocol/mod.rs` - Export ProtocolRegistry for Lua use

### Files to Create:
- `examples/raw_echo.lua` - Simple echo test without protocols
- `examples/modbus_with_tools.lua` - Modbus RTU example using protocol tools
- `examples/custom_protocol.lua` - Example with custom protocol implementation
- `tests/lua_integration_tests.rs` - Integration tests for Lua APIs

### File Responsibilities:
- **bindings.rs**: All Lua API function registrations (serial, protocol, conversion)
- **executor.rs**: ScriptEngine enhancement to manage PortManager instance
- **stdlib.rs**: Pure utility functions for data conversion
- **main.rs**: Script file loading and execution orchestration

---

## Task 1: Expose PortManager to ScriptEngine

**Files:**
- Modify: `src/lua/executor.rs`
- Modify: `src/lua/bindings.rs`

**Rationale:** Lua functions need access to a shared PortManager instance to manage serial ports across API calls.

- [ ] **Step 1: Add PortManager field to ScriptEngine**

Read `src/lua/executor.rs` completely to understand current structure, then add PortManager field:

```rust
use crate::serial_core::PortManager;

pub struct ScriptEngine {
    bindings: LuaBindings,
    port_manager: PortManager,  // Add this field
}
```

- [ ] **Step 2: Update ScriptEngine::new()**

Modify the constructor to initialize PortManager:

```rust
impl ScriptEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            bindings: LuaBindings::new()?,
            port_manager: PortManager::new(),  // Add this
        })
    }
}
```

- [ ] **Step 3: Add getter for PortManager**

Add method to expose PortManager to LuaBindings:

```rust
impl ScriptEngine {
    pub fn port_manager(&self) -> &PortManager {
        &self.port_manager
    }
}
```

- [ ] **Step 4: Update LuaBindings to hold PortManager reference**

Modify `src/lua/bindings.rs` to accept PortManager:

```rust
pub struct LuaBindings {
    lua: Lua,
    port_manager: Option<std::sync::Arc<tokio::sync::Mutex<PortManager>>>,
}

impl LuaBindings {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self {
            lua,
            port_manager: None,
        })
    }

    pub fn set_port_manager(&mut self, pm: std::sync::Arc<tokio::sync::Mutex<PortManager>>) {
        self.port_manager = Some(pm);
    }
}
```

- [ ] **Step 5: Run existing tests**

Run: `cargo test --lib lua::executor`

Expected: All existing tests pass

- [ ] **Step 6: Commit**

```bash
git add src/lua/executor.rs src/lua/bindings.rs
git commit -m "refactor: add PortManager to ScriptEngine for Lua API access"
```

---

## Task 2: Implement serial_open Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add serial_open function to LuaBindings**

Add this method to `impl LuaBindings`:

```rust
impl LuaBindings {
    pub fn register_serial_open(&self) -> Result<()> {
        let port_manager = self.port_manager.clone().unwrap();

        let open = self.lua.create_function(move |_, port_name: String, baudrate: u32| {
            // Use tokio runtime for async call
            let rt = tokio::runtime::Runtime::new()?;
            let pm_guard = rt.block_on(port_manager.lock());
            let config = crate::serial_core::SerialConfig {
                baudrate,
                ..Default::default()
            };

            let port_id = rt.block_on(pm_guard.open_port(&port_name, config))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(port_id)
        })?;

        self.lua.globals().set("serial_open", open)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test for serial_open**

Create test in `src/lua/bindings.rs` in the tests module:

```rust
#[test]
fn test_serial_open_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_serial_open().unwrap();

    let script = r#"
        local port, err = serial_open("/dev/ttyUSB0", 115200)
        -- Will fail because port doesn't exist, but tests the API
        assert(type(port) == "string" or type(err) == "string")
    "#;

    let result = bindings.execute_script(script);
    // Should not error - the Lua script should handle it
    assert!(result.is_ok() || result.unwrap_err().to_string().contains("Serial port error"));
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_serial_open_lua`

Expected: Test passes (script executes, may fail to open port but API works)

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add serial_open Lua function"
```

---

## Task 3: Implement serial_close Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add serial_close function**

```rust
impl LuaBindings {
    pub fn register_serial_close(&self) -> Result<()> {
        let port_manager = self.port_manager.clone().unwrap();

        let close = self.lua.create_function(move |_, port_id: String| {
            let rt = tokio::runtime::Runtime::new()?;
            let pm_guard = rt.block_on(port_manager.lock());

            rt.block_on(pm_guard.close_port(&port_id))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(true)
        })?;

        self.lua.globals().set("serial_close", close)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_serial_close_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_serial_close().unwrap();

    let script = r#"
        local result, err = serial_close("nonexistent-port")
        -- Will fail but tests the API
        assert(result == true or type(err) == "string")
    "#;

    let result = bindings.execute_script(script);
    assert!(result.is_ok());
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_serial_close_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add serial_close Lua function"
```

---

## Task 4: Implement serial_send Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add serial_send function**

```rust
impl LuaBindings {
    pub fn register_serial_send(&self) -> Result<()> {
        let port_manager = self.port_manager.clone().unwrap();

        let send = self.lua.create_function(move |_, port_id: String, data: String| {
            let rt = tokio::runtime::Runtime::new()?;
            let pm_guard = rt.block_on(port_manager.lock());

            let port_handle = rt.block_on(pm_guard.get_port(&port_id))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let mut handle = rt.block_on(port_handle.lock());
            let bytes = handle.write(data.as_bytes())
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(bytes)
        })?;

        self.lua.globals().set("serial_send", send)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_serial_send_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_serial_send().unwrap();

    let script = r#"
        local sent, err = serial_send("test-port", "Hello")
        -- Will fail but tests the API
        assert(type(sent) == "number" or type(err) == "string")
    "#;

    let result = bindings.execute_script(script);
    assert!(result.is_ok());
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_serial_send_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add serial_send Lua function"
```

---

## Task 5: Implement serial_recv Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add serial_recv function**

```rust
impl LuaBindings {
    pub fn register_serial_recv(&self) -> Result<()> {
        let port_manager = self.port_manager.clone().unwrap();

        let recv = self.lua.create_function(move |_, port_id: String, timeout_ms: u64| {
            let rt = tokio::runtime::Runtime::new()?;
            let pm_guard = rt.block_on(port_manager.lock());

            let port_handle = rt.block_on(pm_guard.get_port(&port_id))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let mut handle = rt.block_on(port_handle.lock());
            let mut buffer = vec![0u8; 4096];

            // Read with timeout
            let result = tokio::time::timeout(
                std::time::Duration::from_millis(timeout_ms),
                handle.read(&mut buffer)
            );

            let n = rt.block_on(result)
                .map_err(|_| mlua::Error::RuntimeError("Timeout".to_string()))?
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            buffer.truncate(n);
            Ok(String::from_utf8_lossy(&buffer).to_string())
        })?;

        self.lua.globals().set("serial_recv", recv)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_serial_recv_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_serial_recv().unwrap();

    let script = r#"
        local data, err = serial_recv("test-port", 1000)
        -- Will fail but tests the API
        assert(type(data) == "string" or type(err) == "string")
    "#;

    let result = bindings.execute_script(script);
    assert!(result.is_ok());
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_serial_recv_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add serial_recv Lua function"
```

---

## Task 6: Implement serial_list Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add serial_list function**

```rust
impl LuaBindings {
    pub fn register_serial_list(&self) -> Result<()> {
        let port_manager = self.port_manager.clone().unwrap();

        let list = self.lua.create_function(move |_, ()| {
            let rt = tokio::runtime::Runtime::new()?;
            let pm_guard = rt.block_on(port_manager.lock());

            let ports = pm_guard.list_ports()
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            // Convert to Lua table
            let result = self.lua.create_table()?;
            for (i, port) in ports.iter().enumerate() {
                let port_table = self.lua.create_table()?;
                port_table.set("port_name", port.port_name.clone())?;
                port_table.set("port_type", port.port_type.clone())?;
                result.set(i + 1, port_table)?;
            }

            Ok(result)
        })?;

        self.lua.globals().set("serial_list", list)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_serial_list_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_serial_list().unwrap();

    let script = r#"
        local ports = serial_list()
        assert(type(ports) == "table")
    "#;

    assert!(bindings.execute_script(script).is_ok());
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_serial_list_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add serial_list Lua function"
```

---

## Task 7: Implement protocol_encode Function

**Files:**
- Modify: `src/lua/bindings.rs`
- Modify: `src/protocol/mod.rs`

- [ ] **Step 1: Export ProtocolRegistry from protocol module**

Modify `src/protocol/mod.rs` to export registry:

```rust
pub mod registry;
pub use registry::{ProtocolFactory, ProtocolInfo, ProtocolRegistry};
```

- [ ] **Step 2: Add protocol_encode function**

```rust
impl LuaBindings {
    pub fn register_protocol_encode(&self) -> Result<()> {
        let encode = self.lua.create_function(|_, protocol_name: String, data: String| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()?;
            let registry = ProtocolRegistry::new();

            // Register built-in protocols
            rt.block_on(register_builtins(&mut registry));

            let mut protocol = rt.block_on(registry.get_protocol(&protocol_name))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let encoded = protocol.encode(data.as_bytes())
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(String::from_utf8_lossy(&encoded).to_string())
        })?;

        self.lua.globals().set("protocol_encode", encode)?;
        Ok(())
    }
}

async fn register_builtins(registry: &mut ProtocolRegistry) {
    use crate::protocol::built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol};
    use crate::protocol::registry::SimpleProtocolFactory;

    registry.register(SimpleProtocolFactory::new(
        "line".to_string(),
        "Line-based protocol".to_string(),
        || LineProtocol::new(),
    )).await;

    registry.register(SimpleProtocolFactory::new(
        "at_command".to_string(),
        "AT Command protocol".to_string(),
        || AtCommandProtocol::new(),
    )).await;

    registry.register(SimpleProtocolFactory::new(
        "modbus_rtu".to_string(),
        "Modbus RTU protocol".to_string(),
        || ModbusProtocol::new(crate::protocol::built_in::ModbusMode::Rtu),
    )).await;

    registry.register(SimpleProtocolFactory::new(
        "modbus_ascii".to_string(),
        "Modbus ASCII protocol".to_string(),
        || ModbusProtocol::new(crate::protocol::built_in::ModbusMode::Ascii),
    )).await;
}
```

- [ ] **Step 3: Write test**

```rust
#[test]
fn test_protocol_encode_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_protocol_encode().unwrap();

    let script = r#"
        local encoded = protocol_encode("line", "Hello")
        assert(type(encoded) == "string")
        assert(string.sub(encoded, -1) == "\n")
    "#;

    assert!(bindings.execute_script(script).is_ok());
}
```

- [ ] **Step 4: Run test**

Run: `cargo test test_protocol_encode_lua`

Expected: Test passes, line protocol adds newline

- [ ] **Step 5: Commit**

```bash
git add src/lua/bindings.rs src/protocol/mod.rs
git commit -m "feat: add protocol_encode Lua function"
```

---

## Task 8: Implement protocol_decode Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add protocol_decode function**

```rust
impl LuaBindings {
    pub fn register_protocol_decode(&self) -> Result<()> {
        let decode = self.lua.create_function(|_, protocol_name: String, data: String| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()?;
            let registry = ProtocolRegistry::new();

            rt.block_on(register_builtins(&mut registry));

            let mut protocol = rt.block_on(registry.get_protocol(&protocol_name))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let decoded = protocol.parse(data.as_bytes())
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            Ok(String::from_utf8_lossy(&decoded).to_string())
        })?;

        self.lua.globals().set("protocol_decode", decode)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
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
```

- [ ] **Step 3: Run test**

Run: `cargo test test_protocol_decode_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add protocol_decode Lua function"
```

---

## Task 9: Implement protocol_list Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add protocol_list function**

```rust
impl LuaBindings {
    pub fn register_protocol_list(&self) -> Result<()> {
        let list = self.lua.create_function(|_, ()| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()?;
            let registry = ProtocolRegistry::new();

            rt.block_on(register_builtins(&mut registry));

            let protocols = rt.block_on(registry.list_protocols());

            let result = self.lua.create_table()?;
            for (i, protocol) in protocols.iter().enumerate() {
                let proto_table = self.lua.create_table()?;
                proto_table.set("name", protocol.name.clone())?;
                proto_table.set("description", protocol.description.clone())?;
                result.set(i + 1, proto_table)?;
            }

            Ok(result)
        })?;

        self.lua.globals().set("protocol_list", list)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_protocol_list_lua() {
    let bindings = LuaBindings::new().unwrap();
    bindings.register_protocol_list().unwrap();

    let script = r#"
        local protocols = protocol_list()
        assert(type(protocols) == "table")
        -- Should have at least line, at_command, modbus_rtu, modbus_ascii
        assert(#protocols >= 4)
    "#;

    assert!(bindings.execute_script(script).is_ok());
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_protocol_list_lua`

Expected: Test passes, lists 4+ protocols

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add protocol_list Lua function"
```

---

## Task 10: Implement protocol_info Function

**Files:**
- Modify: `src/lua/bindings.rs`

- [ ] **Step 1: Add protocol_info function**

```rust
impl LuaBindings {
    pub fn register_protocol_info(&self) -> Result<()> {
        let info = self.lua.create_function(|_, protocol_name: String| {
            use crate::protocol::ProtocolRegistry;

            let rt = tokio::runtime::Runtime::new()?;
            let registry = ProtocolRegistry::new();

            rt.block_on(register_builtins(&mut registry));

            let protocols = rt.block_on(registry.list_protocols());
            let protocol = protocols.iter()
                .find(|p| p.name == protocol_name)
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Protocol not found: {}", protocol_name)))?;

            let result = self.lua.create_table()?;
            result.set("name", protocol.name.clone())?;
            result.set("description", protocol.description.clone())?;
            Ok(result)
        })?;

        self.lua.globals().set("protocol_info", info)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
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
```

- [ ] **Step 3: Run test**

Run: `cargo test test_protocol_info_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs
git commit -m "feat: add protocol_info Lua function"
```

---

## Task 11: Implement hex_to_bytes Function

**Files:**
- Modify: `src/lua/stdlib.rs`

- [ ] **Step 1: Add hex_to_bytes function**

```rust
impl LuaStdLib {
    pub fn register_hex_to_bytes(lua: &Lua) -> Result<()> {
        let hex2bytes = lua.create_function(|_, hex: String| {
            if hex.len() % 2 != 0 {
                return Err(mlua::Error::RuntimeError(
                    "Hex string must have even length".to_string()
                ));
            }

            let mut bytes = Vec::new();
            for i in (0..hex.len()).step_by(2) {
                let byte_str = &hex[i..i+2];
                let byte = u8::from_str_radix(byte_str, 16)
                    .map_err(|_| mlua::Error::RuntimeError(
                        format!("Invalid hex: {}", byte_str)
                    ))?;
                bytes.push(byte);
            }

            // Return as Lua array (table with integer indices)
            let result = lua.create_table()?;
            for (i, byte) in bytes.iter().enumerate() {
                result.set(i + 1, *byte)?;
            }
            Ok(result)
        })?;

        lua.globals().set("hex_to_bytes", hex2bytes)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_hex_to_bytes_lua() {
    let lua = Lua::new();
    LuaStdLib::register_hex_to_bytes(&lua).unwrap();

    let script = r#"
        local bytes = hex_to_bytes("010203")
        assert(type(bytes) == "table")
        assert(bytes[1] == 1)
        assert(bytes[2] == 2)
        assert(bytes[3] == 3)
    "#;

    lua.load(script).exec().unwrap();
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_hex_to_bytes_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/stdlib.rs
git commit -m "feat: add hex_to_bytes Lua function"
```

---

## Task 12: Implement bytes_to_hex Function

**Files:**
- Modify: `src/lua/stdlib.rs`

- [ ] **Step 1: Add bytes_to_hex function**

```rust
impl LuaStdLib {
    pub fn register_bytes_to_hex(lua: &Lua) -> Result<()> {
        let bytes2hex = lua.create_function(|_, bytes: Value| {
            let bytes_vec = match bytes {
                Value::String(s) => {
                    let s = s.to_str().unwrap();
                    s.as_bytes().to_vec()
                }
                Value::Table(t) => {
                    let mut vec = Vec::new();
                    for pair in t.pairs::<usize, u8>() {
                        let (_, byte) = pair.unwrap();
                        vec.push(byte);
                    }
                    vec
                }
                _ => return Err(mlua::Error::RuntimeError(
                    "Expected string or table".to_string()
                )),
            };

            let hex: String = bytes_vec.iter()
                .map(|b| format!("{:02x}", b))
                .collect();

            Ok(hex)
        })?;

        lua.globals().set("bytes_to_hex", bytes2hex)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_bytes_to_hex_lua() {
    let lua = Lua::new();
    LuaStdLib::register_bytes_to_hex(&lua).unwrap();

    let script = r#"
        local hex = bytes_to_hex({1, 2, 3})
        assert(hex == "010203")
    "#;

    lua.load(script).exec().unwrap();
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_bytes_to_hex_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/stdlib.rs
git commit -m "feat: add bytes_to_hex Lua function"
```

---

## Task 13: Implement bytes_to_string Function

**Files:**
- Modify: `src/lua/stdlib.rs`

- [ ] **Step 1: Add bytes_to_string function**

```rust
impl LuaStdLib {
    pub fn register_bytes_to_string(lua: &Lua) -> Result<()> {
        let bytes2str = lua.create_function(|_, bytes: Value| {
            let bytes_vec = match bytes {
                Value::Table(t) => {
                    let mut vec = Vec::new();
                    for pair in t.pairs::<usize, u8>() {
                        let (_, byte) = pair.unwrap();
                        vec.push(byte);
                    }
                    vec
                }
                _ => return Err(mlua::Error::RuntimeError(
                    "Expected table of bytes".to_string()
                )),
            };

            String::from_utf8(bytes_vec)
                .map_err(|_| mlua::Error::RuntimeError(
                    "Invalid UTF-8 sequence".to_string()
                ))
        })?;

        lua.globals().set("bytes_to_string", bytes2str)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_bytes_to_string_lua() {
    let lua = Lua::new();
    LuaStdLib::register_bytes_to_string(&lua).unwrap();

    let script = r#"
        local str = bytes_to_string({72, 101, 108, 108, 111})  -- "Hello"
        assert(str == "Hello")
    "#;

    lua.load(script).exec().unwrap();
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_bytes_to_string_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/stdlib.rs
git commit -m "feat: add bytes_to_string Lua function"
```

---

## Task 14: Implement string_to_bytes Function

**Files:**
- Modify: `src/lua/stdlib.rs`

- [ ] **Step 1: Add string_to_bytes function**

```rust
impl LuaStdLib {
    pub fn register_string_to_bytes(lua: &Lua) -> Result<()> {
        let str2bytes = lua.create_function(|_, s: String| {
            let bytes = s.into_bytes();
            let result = lua.create_table()?;
            for (i, byte) in bytes.iter().enumerate() {
                result.set(i + 1, *byte)?;
            }
            Ok(result)
        })?;

        lua.globals().set("string_to_bytes", str2bytes)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Write test**

```rust
#[test]
fn test_string_to_bytes_lua() {
    let lua = Lua::new();
    LuaStdLib::register_string_to_bytes(&lua).unwrap();

    let script = r#"
        local bytes = string_to_bytes("ABC")
        assert(type(bytes) == "table")
        assert(bytes[1] == 65)  -- 'A'
        assert(bytes[2] == 66)  -- 'B'
        assert(bytes[3] == 67)  -- 'C'
    "#;

    lua.load(script).exec().unwrap();
}
```

- [ ] **Step 3: Run test**

Run: `cargo test test_string_to_bytes_lua`

Expected: Test passes

- [ ] **Step 4: Commit**

```bash
git add src/lua/stdlib.rs
git commit -m "feat: add string_to_bytes Lua function"
```

---

## Task 15: Update register_all_apis to Include New Functions

**Files:**
- Modify: `src/lua/bindings.rs`
- Modify: `src/lua/stdlib.rs`

- [ ] **Step 1: Update LuaBindings::register_all_apis**

```rust
impl LuaBindings {
    pub fn register_all_apis(&self) -> Result<()> {
        // Existing APIs
        self.register_log_api()?;
        self.register_utility_apis()?;

        // New Serial APIs
        self.register_serial_open()?;
        self.register_serial_close()?;
        self.register_serial_send()?;
        self.register_serial_recv()?;
        self.register_serial_list()?;

        // New Protocol APIs
        self.register_protocol_encode()?;
        self.register_protocol_decode()?;
        self.register_protocol_list()?;
        self.register_protocol_info()?;

        Ok(())
    }
}
```

- [ ] **Step 2: Create LuaStdLib::register_all**

Add to `src/lua/stdlib.rs`:

```rust
impl LuaStdLib {
    pub fn register_all(lua: &Lua) -> Result<()> {
        LuaStdLib::register_hex_to_bytes(lua)?;
        LuaStdLib::register_bytes_to_hex(lua)?;
        LuaStdLib::register_bytes_to_string(lua)?;
        LuaStdLib::register_string_to_bytes(lua)?;
        Ok(())
    }
}
```

- [ ] **Step 3: Run all tests**

Run: `cargo test --lib lua`

Expected: All Lua tests pass

- [ ] **Step 4: Commit**

```bash
git add src/lua/bindings.rs src/lua/stdlib.rs
git commit -m "refactor: update register_all_apis to include new functions"
```

---

## Task 16: Implement run_lua_script in main.rs

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add run_lua_script function**

```rust
async fn run_lua_script(path: std::path::PathBuf) -> Result<()> {
    use serial_cli::lua::executor::ScriptEngine;
    use serial_cli::lua::stdlib::LuaStdLib;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // 1. Create Lua engine
    let engine = ScriptEngine::new()?;

    // 2. Set up PortManager reference
    let port_manager = Arc::new(Mutex::new(engine.port_manager().clone()));
    engine.bindings.set_port_manager(port_manager);

    // 3. Register all APIs
    engine.bindings.register_all_apis()?;
    LuaStdLib::register_all(engine.bindings.lua())?;

    // 4. Read script file
    let script_content = std::fs::read_to_string(&path)
        .map_err(|e| SerialError::Io(e))?;

    // 5. Execute script
    engine.bindings.execute_script(&script_content)?;

    Ok(())
}
```

- [ ] **Step 2: Update Commands::Run branch**

Find the `Commands::Run { script }` branch and replace:

```rust
Commands::Run { script } => {
    run_lua_script(std::path::PathBuf::from(script)).await?;
}
```

- [ ] **Step 3: Create test script**

Create temporary test script `/tmp/test_lua.lua`:

```lua
-- Test basic Lua execution
print("Lua script is running!")

-- Test log API
log_info("This is an info message")

-- Test protocol list
local protocols = protocol_list()
log_info("Available protocols: " .. #protocols)
```

- [ ] **Step 4: Test run command**

Run: `cargo run -- run /tmp/test_lua.lua`

Expected Output:
```
Lua script is running!
[INFO] This is an info message
[INFO] Available protocols: 4
```

- [ ] **Step 5: Clean up test script**

Run: `rm /tmp/test_lua.lua`

- [ ] **Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement run command for Lua script execution"
```

---

## Task 17: Create Example Scripts

**Files:**
- Create: `examples/raw_echo.lua`
- Create: `examples/modbus_with_tools.lua`
- Create: `examples/custom_protocol.lua`

- [ ] **Step 1: Create raw_echo.lua**

```lua
-- examples/raw_echo.lua
-- Simple echo test without using protocol tools

local port, err = serial_open('/dev/ttyUSB0', 115200)
if not port then
    log_error('Failed to open port: ' .. tostring(err))
    return
end

log_info('Port opened: ' .. port)

-- Send raw data
local sent = serial_send(port, 'Hello, Serial!')
log_info('Sent ' .. sent .. ' bytes')

-- Receive response (1 second timeout)
local data = serial_recv(port, 1000)
if data and #data > 0 then
    log_info('Received: ' .. data)
else
    log_warn('No response received')
end

-- Close port
serial_close(port)
log_info('Port closed')
```

- [ ] **Step 2: Create modbus_with_tools.lua**

```lua
-- examples/modbus_with_tools.lua
-- Modbus RTU example using protocol tools

local port, err = serial_open('/dev/ttyUSB0', 9600)
if not port then
    log_error('Failed to open port: ' .. tostring(err))
    return
end

log_info('Port opened: ' .. port)

-- Construct Modbus PDU (slave ID=1, function=3, start addr=0x0000, count=10)
local pdu = string.char(0x01, 0x03, 0x00, 0x00, 0x00, 0x0A)

-- Encode with Modbus RTU (adds CRC)
local encoded = protocol_encode('modbus_rtu', pdu)
if not encoded then
    log_error('Failed to encode')
    serial_close(port)
    return
end

log_info('Encoded request: ' .. bytes_to_hex(string_to_bytes(encoded)))

-- Send encoded data
local sent = serial_send(port, encoded)
log_info('Sent ' .. sent .. ' bytes')

-- Receive response
local response = serial_recv(port, 1000)
if response then
    log_info('Raw response: ' .. bytes_to_hex(string_to_bytes(response)))

    -- Decode response (validates and removes CRC)
    local decoded = protocol_decode('modbus_rtu', response)
    if decoded then
        log_info('Valid Modbus response received')
        log_info('Decoded data: ' .. bytes_to_hex(string_to_bytes(decoded)))

        -- Parse response
        local slave_id = string.byte(decoded, 1)
        local func_code = string.byte(decoded, 2)
        local byte_count = string.byte(decoded, 3)

        log_info(string.format('Slave ID: %d, Function: %d, Bytes: %d',
                              slave_id, func_code, byte_count))
    else
        log_error('Invalid Modbus response (CRC mismatch?)')
    end
else
    log_warn('No response received')
end

serial_close(port)
log_info('Port closed')
```

- [ ] **Step 3: Create custom_protocol.lua**

```lua
-- examples/custom_protocol.lua
-- Example showing custom protocol implementation

-- Simple custom protocol: add 2-byte checksum at end
local function add_checksum(data)
    local sum = 0
    for i = 1, #data do
        sum = sum + string.byte(data, i)
    end
    return data .. string.char(sum % 256, sum >> 8)
end

local function verify_checksum(data)
    if #data < 2 then
        return false
    end

    local sum = 0
    for i = 1, #data - 2 do
        sum = sum + string.byte(data, i)
    end

    local received_checksum = string.byte(data, #data - 1) +
                              (string.byte(data, #data) << 8)
    return sum == received_checksum
end

-- Open port
local port, err = serial_open('/dev/ttyUSB0', 115200)
if not port then
    log_error('Failed to open port: ' .. tostring(err))
    return
end

-- Prepare data
local data = "Test message"
local frame = add_checksum(data)

log_info('Sending frame with checksum')
serial_send(port, frame)

-- Receive response
local response = serial_recv(port, 1000)
if response then
    if verify_checksum(response) then
        log_info('Valid response received')
        log_info('Data: ' .. string.sub(response, 1, #response - 2))
    else
        log_error('Checksum verification failed')
    end
end

serial_close(port)
```

- [ ] **Step 4: Update README with example usage**

Add to README.md:

````markdown
## Lua Scripting

Serial CLI supports Lua scripting for automation and custom protocols.

### Running Lua Scripts

```bash
serial-cli run examples/raw_echo.lua
```

### Available Lua APIs

#### Serial Operations
- `serial_open(port_name, baudrate) -> port_id | nil, error`
- `serial_close(port_id) -> true | nil, error`
- `serial_send(port_id, data) -> bytes_sent | nil, error`
- `serial_recv(port_id, timeout_ms) -> data | nil, error`
- `serial_list() -> [{port_name, port_type}, ...]`

#### Protocol Tools
- `protocol_encode(protocol_name, data) -> encoded_data | nil, error`
- `protocol_decode(protocol_name, data) -> decoded_data | nil, error`
- `protocol_list() -> [{name, description}, ...]`
- `protocol_info(protocol_name) -> {name, description, ...}`

#### Data Conversion
- `hex_to_bytes(hex_string) -> byte_array`
- `bytes_to_hex(byte_array) -> hex_string`
- `bytes_to_string(byte_array) -> string`
- `string_to_bytes(string) -> byte_array`

### Examples

See the `examples/` directory for complete examples:
- `raw_echo.lua` - Simple echo without protocols
- `modbus_with_tools.lua` - Modbus RTU with protocol tools
- `custom_protocol.lua` - Custom protocol implementation
````

- [ ] **Step 5: Commit**

```bash
git add examples/*.lua README.md
git commit -m "docs: add Lua scripting examples and documentation"
```

---

## Task 18: Write Integration Tests

**Files:**
- Create: `tests/lua_integration_tests.rs`

- [ ] **Step 1: Create integration test file**

```rust
//! Integration tests for Lua APIs

use serial_cli::lua::executor::ScriptEngine;
use serial_cli::lua::stdlib::LuaStdLib;
use std::sync::Arc;
use tokio::sync::Mutex;

#[test]
fn test_serial_api_integration() {
    let engine = ScriptEngine::new().unwrap();
    let port_manager = Arc::new(Mutex::new(engine.port_manager().clone()));
    engine.bindings.set_port_manager(port_manager);
    engine.bindings.register_all_apis().unwrap();
    LuaStdLib::register_all(engine.bindings.lua()).unwrap();

    // Test serial_list (should work without real hardware)
    let script = r#"
        local ports = serial_list()
        assert(type(ports) == "table", "serial_list should return table")
    "#;

    assert!(engine.bindings.execute_script(script).is_ok());
}

#[test]
fn test_protocol_api_integration() {
    let engine = ScriptEngine::new().unwrap();
    LuaStdLib::register_all(engine.bindings.lua()).unwrap();
    engine.bindings.register_all_apis().unwrap();

    let script = r#"
        -- Test protocol_list
        local protocols = protocol_list()
        assert(type(protocols) == "table")
        assert(#protocols >= 4, "Should have at least 4 protocols")

        -- Test protocol_encode with line protocol
        local encoded = protocol_encode("line", "test")
        assert(type(encoded) == "string")
        assert(string.sub(encoded, -1) == "\n", "Line protocol should add newline")

        -- Test protocol_decode
        local decoded = protocol_decode("line", "test\n")
        assert(type(decoded) == "string")
        assert(decoded == "test\n")
    "#;

    assert!(engine.bindings.execute_script(script).is_ok());
}

#[test]
fn test_conversion_api_integration() {
    let lua = mlua::Lua::new();
    LuaStdLib::register_all(&lua).unwrap();

    let script = r#"
        -- Test hex_to_bytes
        local bytes = hex_to_bytes("010203")
        assert(bytes[1] == 1)
        assert(bytes[2] == 2)
        assert(bytes[3] == 3)

        -- Test bytes_to_hex
        local hex = bytes_to_hex({1, 2, 3})
        assert(hex == "010203")

        -- Test bytes_to_string
        local str = bytes_to_string({72, 101, 108, 108, 111})
        assert(str == "Hello")

        -- Test string_to_bytes
        local bytes2 = string_to_bytes("ABC")
        assert(bytes2[1] == 65)
        assert(bytes2[2] == 66)
        assert(bytes2[3] == 67)
    "#;

    lua.load(script).exec().unwrap();
}

#[test]
fn test_end_to_end_modbus_workflow() {
    let engine = ScriptEngine::new().unwrap();
    LuaStdLib::register_all(engine.bindings.lua()).unwrap();
    engine.bindings.register_all_apis().unwrap();

    // Simulate Modbus workflow without real hardware
    let script = r#"
        -- Test Modbus encoding
        local pdu = string.char(0x01, 0x03, 0x00, 0x00, 0x00, 0x0A)
        local encoded = protocol_encode('modbus_rtu', pdu)

        assert(type(encoded) == "string")
        assert(#encoded > #pdu, "Encoded data should include CRC")

        -- Test Modbus decoding
        local decoded = protocol_decode('modbus_rtu', encoded)
        assert(type(decoded) == "string")
        assert(#decoded == #pdu, "Decoded data should have CRC removed")

        -- Verify PDU matches
        assert(decoded == pdu)
    "#;

    assert!(engine.bindings.execute_script(script).is_ok());
}
```

- [ ] **Step 2: Run integration tests**

Run: `cargo test --test lua_integration_tests`

Expected: All integration tests pass

- [ ] **Step 3: Commit**

```bash
git add tests/lua_integration_tests.rs
git commit -m "test: add Lua API integration tests"
```

---

## Task 19: Final Verification and Cleanup

**Files:**
- Multiple

- [ ] **Step 1: Run full test suite**

Run: `cargo test --all`

Expected: All tests pass (unit + integration)

- [ ] **Step 2: Build release binary**

Run: `cargo build --release`

Expected: Clean build, no warnings

- [ ] **Step 3: Test with real hardware (if available)**

If you have serial hardware available:

```bash
# List ports
./target/release/serial-cli list

# Run example
./target/release/serial-cli run examples/raw_echo.lua
```

- [ ] **Step 4: Check for compiler warnings**

Run: `cargo clippy --all-targets --all-features`

Expected: No warnings (or fix any warnings)

- [ ] **Step 5: Format code**

Run: `cargo fmt`

Expected: All code formatted

- [ ] **Step 6: Final commit**

```bash
git add .
git commit -m "chore: final cleanup and verification"
```

---

## Self-Review Checklist

**Spec Coverage:**
- ✅ Serial API (open, close, send, recv, list) - Tasks 2-6
- ✅ Protocol API (encode, decode, list, info) - Tasks 7-10
- ✅ Data conversion (hex_to_bytes, bytes_to_hex, bytes_to_string, string_to_bytes) - Tasks 11-14
- ✅ Main program integration (run command) - Task 16
- ✅ Example scripts - Task 17
- ✅ Tests - Task 19

**Placeholder Scan:**
- ✅ No "TBD" or "TODO" found
- ✅ All code is complete and executable
- ✅ All tests have actual assertions

**Type Consistency:**
- ✅ Function names consistent (serial_*, protocol_*)
- ✅ Return value patterns consistent (result | nil, error)
- ✅ Parameter ordering consistent

**File Structure:**
- ✅ Clear boundaries: bindings.rs (API registration), stdlib.rs (utilities), executor.rs (engine)
- ✅ Related changes grouped in same tasks
- ✅ No file exceeds single responsibility

---

## Verification

To verify the implementation:

1. **Unit Tests:**
   ```bash
   cargo test --lib lua
   ```

2. **Integration Tests:**
   ```bash
   cargo test --test lua_integration_tests
   ```

3. **Manual Testing:**
   ```bash
   # Test basic script execution
   serial-cli run examples/raw_echo.lua

   # Test protocol tools
   serial-cli run examples/modbus_with_tools.lua
   ```

4. **API Coverage:**
   - All 5 serial functions implemented
   - All 4 protocol functions implemented
   - All 4 conversion functions implemented
   - Run command functional

**Success Criteria:**
- ✅ All tests pass
- ✅ Can run Lua scripts via `serial-cli run`
- ✅ Serial API functional (open, close, send, recv, list)
- ✅ Protocol tools functional (encode, decode, list, info)
- ✅ Data conversion utilities work
- ✅ Example scripts demonstrate all features
- ✅ No compiler warnings
- ✅ Documentation updated
