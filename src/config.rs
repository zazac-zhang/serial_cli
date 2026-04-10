//! Configuration management
//!
//! This module handles loading and managing configuration from TOML files.

use crate::error::{Result, SerialError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Global configuration manager
pub struct ConfigManager {
    config: Arc<RwLock<Config>>,
    config_path: Option<PathBuf>,
}

impl ConfigManager {
    /// Create a new config manager with default configuration
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            config_path: None,
        }
    }

    /// Load configuration from a specific path
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let config = load_config(path)?;
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: Some(path.to_path_buf()),
        })
    }

    /// Load configuration with fallback
    pub fn load_with_fallback() -> Self {
        let config = load_config_with_fallback();
        Self {
            config: Arc::new(RwLock::new(config)),
            config_path: get_config_file_path(),
        }
    }

    /// Get the current configuration
    pub fn get(&self) -> Config {
        self.config.read().unwrap().clone()
    }

    /// Update a configuration value by key
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut config = self.config.write().unwrap();

        // Parse the key (e.g., "serial.baudrate", "logging.level")
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["serial", "baudrate"] => {
                let baudrate = value
                    .parse::<u32>()
                    .map_err(|_| SerialError::Config(format!("Invalid baudrate: {}", value)))?;
                config.serial.baudrate = baudrate;
            }
            ["serial", "databits"] => {
                let databits = value
                    .parse::<u8>()
                    .map_err(|_| SerialError::Config(format!("Invalid databits: {}", value)))?;
                config.serial.databits = databits;
            }
            ["serial", "stopbits"] => {
                let stopbits = value
                    .parse::<u8>()
                    .map_err(|_| SerialError::Config(format!("Invalid stopbits: {}", value)))?;
                config.serial.stopbits = stopbits;
            }
            ["serial", "parity"] => {
                config.serial.parity = value.to_string();
            }
            ["serial", "timeout_ms"] => {
                let timeout = value
                    .parse::<u64>()
                    .map_err(|_| SerialError::Config(format!("Invalid timeout: {}", value)))?;
                config.serial.timeout_ms = timeout;
            }
            ["logging", "level"] => {
                config.logging.level = value.to_string();
            }
            ["logging", "format"] => {
                config.logging.format = value.to_string();
            }
            ["logging", "file"] => {
                config.logging.file = value.to_string();
            }
            ["lua", "memory_limit_mb"] => {
                let limit = value
                    .parse::<usize>()
                    .map_err(|_| SerialError::Config(format!("Invalid memory limit: {}", value)))?;
                config.lua.memory_limit_mb = limit;
            }
            ["lua", "timeout_seconds"] => {
                let timeout = value
                    .parse::<u64>()
                    .map_err(|_| SerialError::Config(format!("Invalid timeout: {}", value)))?;
                config.lua.timeout_seconds = timeout;
            }
            ["lua", "enable_sandbox"] => {
                let enable = value
                    .parse::<bool>()
                    .map_err(|_| SerialError::Config(format!("Invalid boolean: {}", value)))?;
                config.lua.enable_sandbox = enable;
            }
            ["task", "max_concurrent"] => {
                let max = value.parse::<usize>().map_err(|_| {
                    SerialError::Config(format!("Invalid max concurrent: {}", value))
                })?;
                config.task.max_concurrent = max;
            }
            ["task", "default_timeout_seconds"] => {
                let timeout = value
                    .parse::<u64>()
                    .map_err(|_| SerialError::Config(format!("Invalid timeout: {}", value)))?;
                config.task.default_timeout_seconds = timeout;
            }
            ["output", "json_pretty"] => {
                let pretty = value
                    .parse::<bool>()
                    .map_err(|_| SerialError::Config(format!("Invalid boolean: {}", value)))?;
                config.output.json_pretty = pretty;
            }
            ["output", "show_timestamp"] => {
                let show = value
                    .parse::<bool>()
                    .map_err(|_| SerialError::Config(format!("Invalid boolean: {}", value)))?;
                config.output.show_timestamp = show;
            }
            _ => {
                return Err(SerialError::Config(format!(
                    "Unknown configuration key: {}",
                    key
                )));
            }
        }

        Ok(())
    }

    /// Save configuration to file
    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let config = self.config.read().unwrap();
        let default_path = PathBuf::from(".serial-cli.toml");
        let save_path = path.unwrap_or_else(|| self.config_path.as_ref().unwrap_or(&default_path));

        // Create parent directories if they don't exist
        if let Some(parent) = save_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    SerialError::Config(format!("Failed to create config directory: {}", e))
                })?;
            }
        }

        // Serialize to TOML
        let toml_str = toml::to_string_pretty(&*config)
            .map_err(|e| SerialError::Config(format!("Failed to serialize config: {}", e)))?;

        // Write to file
        fs::write(save_path, toml_str)
            .map_err(|e| SerialError::Config(format!("Failed to write config file: {}", e)))?;

        tracing::info!("Configuration saved to: {}", save_path.display());
        Ok(())
    }

    /// Reset configuration to defaults
    pub fn reset(&self) -> Result<()> {
        let mut config = self.config.write().unwrap();
        *config = Config::default();
        tracing::info!("Configuration reset to defaults");
        Ok(())
    }

    /// Validate the current configuration
    pub fn validate(&self) -> Result<()> {
        let config = self.config.read().unwrap();

        // Validate serial configuration
        if config.serial.baudrate == 0 {
            return Err(SerialError::Config("Baudrate cannot be zero".to_string()));
        }

        if config.serial.databits < 5 || config.serial.databits > 8 {
            return Err(SerialError::Config(
                "Databits must be between 5 and 8".to_string(),
            ));
        }

        if config.serial.stopbits < 1 || config.serial.stopbits > 2 {
            return Err(SerialError::Config("Stopbits must be 1 or 2".to_string()));
        }

        // Validate parity
        match config.serial.parity.to_lowercase().as_str() {
            "none" | "odd" | "even" => {}
            _ => {
                return Err(SerialError::Config(
                    "Parity must be 'none', 'odd', or 'even'".to_string(),
                ))
            }
        }

        // Validate logging level
        match config.logging.level.to_lowercase().as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => {}
            _ => return Err(SerialError::Config("Invalid logging level".to_string())),
        }

        // Validate task configuration
        if config.task.max_concurrent == 0 {
            return Err(SerialError::Config(
                "Max concurrent tasks cannot be zero".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the actual config file path that would be used
fn get_config_file_path() -> Option<PathBuf> {
    if Path::new(".serial-cli.toml").exists() {
        return Some(PathBuf::from(".serial-cli.toml"));
    }

    let global_path = get_global_config_path()?;
    if global_path.exists() {
        Some(global_path)
    } else {
        None
    }
}

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
    /// Protocol configuration
    #[serde(default)]
    pub protocols: ProtocolsConfig,
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

/// Custom protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProtocolConfig {
    /// Protocol name
    pub name: String,
    /// Path to Lua script file
    pub path: PathBuf,
    /// Protocol version
    #[serde(default)]
    pub version: u64,
    /// Load timestamp
    #[serde(default)]
    pub loaded_at: Option<String>,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolsConfig {
    /// Custom protocols
    #[serde(default)]
    pub custom: HashMap<String, CustomProtocolConfig>,
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
