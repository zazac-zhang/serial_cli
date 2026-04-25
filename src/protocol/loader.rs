//! Lua protocol script loader
//!
//! Loads and initializes Lua protocol scripts from disk, extracting metadata
//! and creating factories for on-demand instantiation.

use crate::error::{ProtocolError, Result, SerialError};
use crate::protocol::lua_ext::create_lua_protocol;
use crate::protocol::validator::ProtocolValidator;
use crate::protocol::{Protocol, ProtocolFactory};
use std::fs;
use std::path::Path;
use std::sync::Arc;

/// Metadata extracted from a loaded Lua protocol script.
#[derive(Debug, Clone)]
pub struct LoadedProtocol {
    /// Protocol name (from `-- Protocol: <name>` comment or filename).
    pub name: String,
    /// Absolute or relative path to the script file.
    pub script_path: std::path::PathBuf,
    /// Full script source code.
    pub script_content: String,
    /// Timestamp when the protocol was loaded.
    pub loaded_at: std::time::SystemTime,
}

/// Lua protocol loader
pub struct ProtocolLoader;

impl ProtocolLoader {
    /// Load and validate a protocol from a Lua script file.
    ///
    /// Validates syntax and required functions (`on_frame`, `on_encode`),
    /// then extracts the protocol name from a `-- Protocol: <name>` comment
    /// or falls back to the filename stem.
    ///
    /// # Errors
    ///
    /// Propagates validation errors from [`ProtocolValidator`] or file I/O errors.
    pub fn load_from_file(path: &Path) -> Result<LoadedProtocol> {
        // Validate the script first
        let validation = ProtocolValidator::validate_script(path)?;

        // Read the script content
        let script_content = fs::read_to_string(path).map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Failed to read script: {}",
                e
            )))
        })?;

        // Extract protocol name
        let name = if let Some(protocol_name) = validation.protocol_name {
            protocol_name
        } else {
            // Use filename without extension
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()
        };

        Ok(LoadedProtocol {
            name: name.clone(),
            script_path: path.to_path_buf(),
            script_content,
            loaded_at: std::time::SystemTime::now(),
        })
    }

    /// Create a [`ProtocolFactory`] that produces [`LuaProtocol`] instances
    /// from the loaded script content.
    ///
    /// # Errors
    ///
    /// Returns an error if the Lua protocol cannot be constructed
    /// (e.g., mlua context creation failure).
    pub fn create_factory(loaded: &LoadedProtocol) -> Result<Arc<dyn ProtocolFactory>> {
        struct LuaProtocolFactory {
            name: String,
            script: String,
        }

        impl ProtocolFactory for LuaProtocolFactory {
            fn create(&self) -> Result<Box<dyn Protocol>> {
                create_lua_protocol(self.name.clone(), &self.script)
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn description(&self) -> &str {
                "Custom Lua protocol"
            }
        }

        Ok(Arc::new(LuaProtocolFactory {
            name: loaded.name.clone(),
            script: loaded.script_content.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_protocol() {
        let script = r#"
            -- Protocol: test_proto
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let loaded = ProtocolLoader::load_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded.name, "test_proto");
        assert_eq!(loaded.script_path, temp_file.path());
        assert!(loaded.script_content.contains("on_frame"));
    }

    #[test]
    fn test_load_protocol_without_name() {
        let script = r#"
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        let file_name = temp_file
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let loaded = ProtocolLoader::load_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded.name, file_name);
    }

    #[test]
    fn test_load_invalid_script() {
        let script = r#"
            function on_frame(data
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolLoader::load_from_file(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_create_factory() {
        let script = r#"
            -- Protocol: factory_test
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let loaded = ProtocolLoader::load_from_file(temp_file.path()).unwrap();
        let factory = ProtocolLoader::create_factory(&loaded).unwrap();

        assert_eq!(factory.name(), "factory_test");
        assert_eq!(factory.description(), "Custom Lua protocol");

        // Test creating protocol instance
        let protocol = factory.create().unwrap();
        assert_eq!(protocol.name(), "factory_test");
    }
}
