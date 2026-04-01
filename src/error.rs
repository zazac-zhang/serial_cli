//! Error types for serial-cli
//!
//! This module defines all error types used throughout the application.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for serial-cli operations
pub type Result<T> = std::result::Result<T, SerialError>;

/// Main error type for serial-cli
#[derive(Error, Debug)]
pub enum SerialError {
    /// Serial port related errors
    #[error("Serial port error: {0}")]
    Serial(#[from] SerialPortError),

    /// Protocol related errors
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    /// Script related errors
    #[error("Script error: {0}")]
    Script(#[from] ScriptError),

    /// Task related errors
    #[error("Task error: {0}")]
    Task(#[from] TaskError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Lua errors
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),
}

/// Serial port specific errors
#[derive(Error, Debug)]
pub enum SerialPortError {
    /// Port not found
    #[error("Port '{0}' not found")]
    PortNotFound(String),

    /// Permission denied
    #[error("Permission denied for port '{0}'")]
    PermissionDenied(String),

    /// Timeout
    #[error("Operation timeout on port '{0}'")]
    Timeout(String),

    /// Port busy
    #[error("Port '{0}' is already in use")]
    PortBusy(String),

    /// Invalid configuration
    #[error("Invalid port configuration: {0}")]
    InvalidConfig(String),

    /// General I/O error
    #[error("Serial I/O error: {0}")]
    IoError(String),
}

/// Protocol specific errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    /// Protocol not found
    #[error("Protocol '{0}' not found")]
    NotFound(String),

    /// Invalid frame format
    #[error("Invalid frame format: {0}")]
    InvalidFrame(String),

    /// Checksum mismatch
    #[error("Checksum mismatch (expected: {expected}, got: {got})")]
    ChecksumFailed { expected: String, got: String },

    /// Unexpected response
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    /// Timeout waiting for response
    #[error("Protocol timeout: {0}")]
    Timeout(String),

    /// Invalid protocol state
    #[error("Invalid protocol state: {0}")]
    InvalidState(String),
}

/// Script (Lua) specific errors
#[derive(Error, Debug)]
pub enum ScriptError {
    /// Syntax error in script
    #[error("Syntax error in {script}:{line}: {message}")]
    Syntax {
        script: PathBuf,
        line: usize,
        message: String,
    },

    /// Runtime error
    #[error("Runtime error in {script}: {message}")]
    Runtime { script: PathBuf, message: String },

    /// API error
    #[error("Script API error: {0}")]
    ApiError(String),

    /// Script not found
    #[error("Script not found: {0}")]
    NotFound(PathBuf),

    /// Sandbox violation
    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),

    /// Resource limit exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
}

/// Task specific errors
#[derive(Error, Debug)]
pub enum TaskError {
    /// Task timeout
    #[error("Task '{0}' timed out after {1}s")]
    Timeout(String, u64),

    /// Dependency failed
    #[error("Task dependency '{0}' failed")]
    DependencyFailed(String),

    /// Resource exhausted
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    /// Task cancelled
    #[error("Task '{0}' was cancelled")]
    Cancelled(String),

    /// Deadlock detected
    #[error("Deadlock detected in task '{0}'")]
    Deadlock(String),

    /// Invalid task state
    #[error("Invalid task state: {0}")]
    InvalidState(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SerialError::Serial(SerialPortError::PortNotFound("COM1".to_string()));
        assert_eq!(err.to_string(), "Serial port error: Port 'COM1' not found");
    }

    #[test]
    fn test_protocol_error() {
        let err = ProtocolError::ChecksumFailed {
            expected: "0x1234".to_string(),
            got: "0x5678".to_string(),
        };
        assert!(err.to_string().contains("Checksum mismatch"));
    }
}
