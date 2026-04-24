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

    /// Virtual port errors
    #[error("Virtual port error: {0}")]
    VirtualPort(String),

    /// Unsupported backend for this platform
    #[error("Unsupported backend for this platform: {0}")]
    UnsupportedBackend(String),

    /// Missing required dependency
    #[error("Missing required dependency: {0}\nHint: {1}")]
    MissingDependency(String, String),

    /// Backend initialization failed
    #[error("Backend initialization failed: {0}")]
    BackendInitFailed(String),
}

/// Serial port specific errors
#[derive(Error, Debug)]
pub enum SerialPortError {
    /// Port not found
    #[error("Port '{0}' not found")]
    PortNotFound(String),

    /// Permission denied
    #[error("{0}")]
    PermissionDeniedWithHelp(String),

    /// Timeout
    #[error("Operation timeout on port '{0}'")]
    Timeout(String),

    /// Port busy
    #[error("{0}")]
    PortBusyWithHelp(String),

    /// Invalid configuration
    #[error("Invalid port configuration: {0}")]
    InvalidConfig(String),

    /// General I/O error
    #[error("Serial I/O error: {0}")]
    IoError(String),
}

impl SerialPortError {
    pub fn permission_denied(port: &str, help: Option<&str>) -> Self {
        let msg = format!("Permission denied for port '{}'", port);
        let msg = if let Some(help) = help {
            format!("{}: {}", msg, help)
        } else {
            msg
        };
        SerialPortError::PermissionDeniedWithHelp(msg)
    }

    pub fn port_busy(port: &str, help: Option<&str>) -> Self {
        let msg = format!("Port '{}' is already in use", port);
        let msg = if let Some(help) = help {
            format!("{}: {}", msg, help)
        } else {
            msg
        };
        SerialPortError::PortBusyWithHelp(msg)
    }
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

    #[test]
    fn test_serial_port_error_variants() {
        let not_found = SerialError::Serial(SerialPortError::PortNotFound("COM1".to_string()));
        assert!(not_found.to_string().contains("COM1"));

        let permission = SerialError::Serial(SerialPortError::permission_denied(
            "/dev/ttyUSB0",
            Some("run as root"),
        ));
        assert!(permission.to_string().contains("/dev/ttyUSB0"));

        let timeout = SerialError::Serial(SerialPortError::Timeout("/dev/ttyS0".to_string()));
        assert!(timeout.to_string().contains("timeout"));

        let busy = SerialError::Serial(SerialPortError::port_busy("COM3", Some("close other app")));
        assert!(busy.to_string().contains("already in use"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let serial_err: SerialError = io_err.into();
        assert!(serial_err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_protocol_error_variants() {
        let not_found = ProtocolError::NotFound("unknown".to_string());
        assert!(not_found.to_string().contains("unknown"));

        let invalid_frame = ProtocolError::InvalidFrame("bad frame".to_string());
        assert!(invalid_frame.to_string().contains("Invalid frame"));

        let unexpected = ProtocolError::UnexpectedResponse("bad response".to_string());
        assert!(unexpected.to_string().contains("Unexpected response"));

        let timeout = ProtocolError::Timeout("read timeout".to_string());
        assert!(timeout.to_string().contains("Protocol timeout"));

        let invalid_state = ProtocolError::InvalidState("closed".to_string());
        assert!(invalid_state.to_string().contains("Invalid protocol state"));
    }

    #[test]
    fn test_script_error_variants() {
        let path = PathBuf::from("test.lua");
        let syntax = ScriptError::Syntax {
            script: path.clone(),
            line: 10,
            message: "unexpected symbol".to_string(),
        };
        assert!(syntax.to_string().contains("test.lua:10"));

        let runtime = ScriptError::Runtime {
            script: path.clone(),
            message: "attempt to call nil".to_string(),
        };
        assert!(runtime.to_string().contains("test.lua"));

        let api = ScriptError::ApiError("invalid call".to_string());
        assert!(api.to_string().contains("Script API error"));

        let not_found = ScriptError::NotFound(PathBuf::from("missing.lua"));
        assert!(not_found.to_string().contains("missing.lua"));

        let sandbox = ScriptError::SandboxViolation("os.execute".to_string());
        assert!(sandbox.to_string().contains("Sandbox violation"));

        let limit = ScriptError::ResourceLimitExceeded("memory".to_string());
        assert!(limit.to_string().contains("Resource limit exceeded"));
    }

    #[test]
    fn test_task_error_variants() {
        let timeout = TaskError::Timeout("task_1".to_string(), 30);
        assert!(timeout.to_string().contains("task_1"));
        assert!(timeout.to_string().contains("30s"));

        let dep_failed = TaskError::DependencyFailed("dep_task".to_string());
        assert!(dep_failed.to_string().contains("dep_task"));

        let exhausted = TaskError::ResourceExhausted("memory".to_string());
        assert!(exhausted.to_string().contains("Resource exhausted"));

        let cancelled = TaskError::Cancelled("task_2".to_string());
        assert!(cancelled.to_string().contains("cancelled"));

        let deadlock = TaskError::Deadlock("task_3".to_string());
        assert!(deadlock.to_string().contains("Deadlock"));

        let invalid = TaskError::InvalidState("running".to_string());
        assert!(invalid.to_string().contains("Invalid task state"));
    }
}
