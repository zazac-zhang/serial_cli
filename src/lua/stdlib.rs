//! Lua standard library
//!
//! This module provides standard library functions for Lua scripts.

use crate::error::Result;
use mlua::Lua;

/// Lua standard library
pub struct LuaStdLib {
    lua: Lua,
}

impl LuaStdLib {
    /// Create a new Lua standard library
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// Register all standard library functions
    pub fn register_all(&self) -> Result<()> {
        self.register_string_utils()?;
        self.register_hex_utils()?;
        self.register_time_utils()?;
        Ok(())
    }

    /// Register string utility functions
    fn register_string_utils(&self) -> Result<()> {
        let globals = self.lua.globals();

        // string.to_hex
        let to_hex = self.lua.create_function(|_, data: String| {
            Ok(data
                .bytes()
                .map(|b| format!("{:02x}", b))
                .collect::<String>())
        })?;
        globals.set("string_to_hex", to_hex)?;

        // string.from_hex
        let from_hex = self.lua.create_function(|_, hex: String| {
            if !hex.len().is_multiple_of(2) {
                return Err(mlua::Error::RuntimeError(
                    "Hex string must have even length".to_string(),
                ));
            }

            let mut bytes = Vec::new();
            for i in (0..hex.len()).step_by(2) {
                let byte_str = &hex[i..i + 2];
                let byte = u8::from_str_radix(byte_str, 16)
                    .map_err(|_| mlua::Error::RuntimeError("Invalid hex string".to_string()))?;
                bytes.push(byte);
            }

            String::from_utf8(bytes)
                .map_err(|_| mlua::Error::RuntimeError("Invalid UTF-8".to_string()))
        })?;
        globals.set("string_from_hex", from_hex)?;

        Ok(())
    }

    /// Register hex utility functions
    fn register_hex_utils(&self) -> Result<()> {
        let globals = self.lua.globals();

        // hex.encode
        let encode = self.lua.create_function(|_, data: Vec<u8>| {
            Ok(data
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>())
        })?;
        globals.set("hex_encode", encode)?;

        // hex.decode
        let decode = self.lua.create_function(|_, hex: String| {
            if !hex.len().is_multiple_of(2) {
                return Err(mlua::Error::RuntimeError(
                    "Hex string must have even length".to_string(),
                ));
            }

            let mut bytes = Vec::new();
            for i in (0..hex.len()).step_by(2) {
                let byte_str = &hex[i..i + 2];
                let byte = u8::from_str_radix(byte_str, 16)
                    .map_err(|_| mlua::Error::RuntimeError("Invalid hex string".to_string()))?;
                bytes.push(byte);
            }

            Ok(bytes)
        })?;
        globals.set("hex_decode", decode)?;

        // hex_to_bytes - converts hex string to Lua byte array (table)
        let hex_to_bytes = self.lua.create_function(|lua, hex: String| {
            if !hex.len().is_multiple_of(2) {
                return Err(mlua::Error::RuntimeError(
                    "Hex string must have even length".to_string(),
                ));
            }

            let mut bytes = Vec::new();
            for i in (0..hex.len()).step_by(2) {
                let byte_str = &hex[i..i + 2];
                let byte = u8::from_str_radix(byte_str, 16).map_err(|_| {
                    mlua::Error::RuntimeError(format!("Invalid hex: {}", byte_str))
                })?;
                bytes.push(byte);
            }

            // Return as Lua array (table with integer indices)
            let result = lua.create_table()?;
            for (i, byte) in bytes.iter().enumerate() {
                result.set(i + 1, *byte)?;
            }
            Ok(result)
        })?;
        globals.set("hex_to_bytes", hex_to_bytes)?;

        Ok(())
    }

    /// Register time utility functions
    fn register_time_utils(&self) -> Result<()> {
        let globals = self.lua.globals();

        // time.sleep (milliseconds)
        let sleep = self.lua.create_function(|_, ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        })?;
        globals.set("sleep_ms", sleep)?;

        // time.now
        let now = self.lua.create_function(|_, _: ()| {
            Ok(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0))
        })?;
        globals.set("time_now", now)?;

        Ok(())
    }
}

impl Default for LuaStdLib {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdlib_creation() {
        let stdlib = LuaStdLib::new().unwrap();
        assert!(stdlib.register_all().is_ok());
    }

    #[test]
    fn test_string_to_hex() {
        let stdlib = LuaStdLib::new().unwrap();
        stdlib.register_all().unwrap();

        let script = r#"
            local result = string_to_hex("AB")
            assert(result == "4142", "Expected 4142, got " .. result)
        "#;

        assert!(stdlib.lua.load(script).exec().is_ok());
    }

    #[test]
    fn test_hex_encode() {
        let stdlib = LuaStdLib::new().unwrap();
        stdlib.register_all().unwrap();

        let script = r#"
            local result = hex_encode({0x41, 0x42})
            assert(result == "4142", "Expected 4142, got " .. result)
        "#;

        assert!(stdlib.lua.load(script).exec().is_ok());
    }

    #[test]
    fn test_hex_to_bytes_lua() {
        let stdlib = LuaStdLib::new().unwrap();
        stdlib.register_all().unwrap();

        let script = r#"
            local bytes = hex_to_bytes("010203")
            assert(type(bytes) == "table")
            assert(bytes[1] == 1)
            assert(bytes[2] == 2)
            assert(bytes[3] == 3)
        "#;

        assert!(stdlib.lua.load(script).exec().is_ok());
    }
}
