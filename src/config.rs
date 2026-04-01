//! Configuration management
//!
//! This module handles loading and managing configuration from TOML files.

use crate::error::{Result, SerialError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Serial port configuration
    pub serial: SerialConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Lua runtime configuration
    pub lua: LuaConfig,
    /// Task scheduler configuration
    pub task: TaskConfig,
    /// Output configuration
    pub output: OutputConfig,
}

/// Serial port configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    /// Default baud rate
    pub baudrate: u32,
    /// Default data bits
    pub databits: u8,
    /// Default stop bits
    pub stopbits: u8,
    /// Default parity
    pub parity: String,
    /// Default timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            databits: 8,
            stopbits: 1,
            parity: "none".to_string(),
            timeout_ms: 1000,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Log format (text or json)
    pub format: String,
    /// Log file path (empty for stdout)
    pub file: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "text".to_string(),
            file: String::new(),
        }
    }
}

/// Lua runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaConfig {
    /// Memory limit in MB
    pub memory_limit_mb: usize,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable sandbox mode
    pub enable_sandbox: bool,
}

impl Default for LuaConfig {
    fn default() -> Self {
        Self {
            memory_limit_mb: 128,
            timeout_seconds: 300,
            enable_sandbox: true,
        }
    }
}

/// Task scheduler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Default timeout in seconds
    pub default_timeout_seconds: u64,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 10,
            default_timeout_seconds: 60,
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Pretty print JSON
    pub json_pretty: bool,
    /// Show timestamps
    pub show_timestamp: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            json_pretty: true,
            show_timestamp: true,
        }
    }
}

/// Load configuration from a file
pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| SerialError::Config(format!("Failed to read config file: {}", e)))?;

    let config: Config = toml::from_str(&content)
        .map_err(|e| SerialError::Config(format!("Failed to parse config: {}", e)))?;

    Ok(config)
}

/// Get global config directory path
pub fn get_global_config_dir() -> Option<PathBuf> {
    let config_dir = if cfg!(windows) {
        std::env::var("APPDATA")
            .ok()
            .map(|p| PathBuf::from(p).join("serial-cli"))
    } else {
        directories::BaseDirs::new().map(|dirs| PathBuf::from(dirs.config_dir()).join("serial-cli"))
    };

    config_dir
}

/// Get global config file path
pub fn get_global_config_path() -> Option<PathBuf> {
    get_global_config_dir().map(|dir| dir.join("config.toml"))
}

/// Load configuration with fallback to defaults
pub fn load_config_with_fallback() -> Config {
    // Try project-level config first
    if let Ok(config) = load_config(Path::new(".serial-cli.toml")) {
        return config;
    }

    // Try global config
    if let Some(path) = get_global_config_path() {
        if path.exists() {
            if let Ok(config) = load_config(&path) {
                return config;
            }
        }
    }

    // Return default config
    Config::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.serial.baudrate, 115200);
        assert_eq!(config.serial.databits, 8);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_serialize_config() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("baudrate"));
    }
}
