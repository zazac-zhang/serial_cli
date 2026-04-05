//! Lua protocol script validator
//!
//! Validates Lua scripts for protocol implementation before loading.

use crate::error::{ProtocolError, Result, SerialError};
use mlua::Lua;
use std::path::Path;

/// Lua script validator
pub struct ProtocolValidator;

impl ProtocolValidator {
    /// Validate a Lua protocol script
    ///
    /// Checks:
    /// - File exists and is readable
    /// - Lua syntax is valid
    /// - Required functions exist (on_frame, on_encode)
    pub fn validate_script(path: &Path) -> Result<ValidationResult> {
        // Check file exists
        if !path.exists() {
            return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                format!("File not found: {:?}", path),
            )));
        }

        // Read file content
        let script = std::fs::read_to_string(path).map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Failed to read file: {}",
                e
            )))
        })?;

        // Create Lua instance
        let lua = Lua::new();

        // Validate syntax
        lua.load(&script).exec().map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Syntax error: {}",
                e
            )))
        })?;

        // Load the script again for validation checks
        lua.load(&script).exec().map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Failed to load script: {}",
                e
            )))
        })?;

        // Validate required functions
        let validation_code = r#"
            local missing = {}

            if type(on_frame) ~= 'function' then
                table.insert(missing, 'on_frame')
            end

            if type(on_encode) ~= 'function' then
                table.insert(missing, 'on_encode')
            end

            if #missing > 0 then
                error('Missing required functions: ' .. table.concat(missing, ', '))
            end

            return true
        "#;

        lua.load(validation_code).eval::<bool>().map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Validation error: {}",
                e
            )))
        })?;

        Ok(ValidationResult {
            protocol_name: Self::extract_protocol_name(&script),
            valid: true,
        })
    }

    /// Extract protocol name from script (looks for specific pattern)
    fn extract_protocol_name(script: &str) -> Option<String> {
        // Try to find: -- Protocol: <name>
        for line in script.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("-- Protocol:") {
                let name = trimmed.trim_start_matches("-- Protocol:").trim();
                return Some(name.to_string());
            }
        }
        None
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub protocol_name: Option<String>,
    pub valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_protocol() {
        let script = r#"
            -- Protocol: test_protocol
            function on_frame(data)
                return data
            end

            function on_encode(data)
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path()).unwrap();
        assert!(result.valid);
        assert_eq!(result.protocol_name, Some("test_protocol".to_string()));
    }

    #[test]
    fn test_missing_on_frame() {
        let script = r#"
            function on_encode(data)
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing required functions"));
    }

    #[test]
    fn test_missing_on_encode() {
        let script = r#"
            function on_frame(data)
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing required functions"));
    }

    #[test]
    fn test_syntax_error() {
        let script = r#"
            function on_frame(data
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Syntax error"));
    }

    #[test]
    fn test_file_not_found() {
        let result = ProtocolValidator::validate_script(Path::new("/nonexistent/file.lua"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("File not found"));
    }

    #[test]
    fn test_extract_protocol_name() {
        let script = r#"
            -- Protocol: my_custom_protocol
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path()).unwrap();
        assert_eq!(result.protocol_name, Some("my_custom_protocol".to_string()));
    }

    #[test]
    fn test_no_protocol_name() {
        let script = r#"
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = ProtocolValidator::validate_script(temp_file.path()).unwrap();
        assert_eq!(result.protocol_name, None);
    }
}
