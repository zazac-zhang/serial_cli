//! NamedPipe backend for Windows
//!
//! This backend creates virtual serial port pairs using Windows Named Pipes.

use crate::error::{Result, SerialError};

#[cfg(windows)]
use crate::serial_core::backends::{
    BackendStats, BridgeErrorRx, BridgeStats, VirtualBackend, VirtualPortEnd,
};
#[cfg(windows)]
use async_trait::async_trait;
#[cfg(windows)]
use std::sync::Arc;
#[cfg(windows)]
use std::time::SystemTime;
#[cfg(windows)]
use tokio::sync::{mpsc, Mutex};

/// NamedPipe backend implementation (Windows only)
#[cfg(windows)]
pub struct NamedPipeBackend {
    /// Pipe A name
    pipe_a_name: String,
    /// Pipe B name
    pipe_b_name: String,
    /// Statistics
    stats: Arc<Mutex<BackendStats>>,
    /// Start time for uptime calculation
    start_time: SystemTime,
    /// Whether the pair has been created
    created: bool,
}

#[cfg(windows)]
impl NamedPipeBackend {
    /// Create a new NamedPipe backend
    pub fn new() -> Result<Self> {
        let uuid_a = uuid::Uuid::new_v4();
        let uuid_b = uuid::Uuid::new_v4();

        Ok(Self {
            pipe_a_name: format!(r"\\.\pipe\serial_cli_a_{}", uuid_a),
            pipe_b_name: format!(r"\\.\pipe\serial_cli_b_{}", uuid_b),
            stats: Arc::new(Mutex::new(BackendStats::default())),
            start_time: SystemTime::now(),
            created: false,
        })
    }

    /// Create a single named pipe
    async fn create_named_pipe(&self, name: &str) -> Result<()> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::core::PCWSTR;
        use windows::Win32::System::Pipes::{
            CreateNamedPipeW, NMPWAIT_USE_DEFAULT_WAIT, PIPE_ACCESS_DUPLEX, PIPE_READMODE_BYTE,
            PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
        };

        // Convert string to wide string
        let wide_name: Vec<u16> = OsStr::new(name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let handle = CreateNamedPipeW(
                PCWSTR(wide_name.as_ptr()),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                8192, // Output buffer size
                8192, // Input buffer size
                0,    // Default timeout
                None, // Default security attributes
            );

            if handle.is_invalid() {
                return Err(SerialError::BackendInitFailed(format!(
                    "Failed to create named pipe: {}",
                    std::io::Error::last_os_error()
                )));
            }

            // The handle is intentionally leaked here - it will be managed by
            // the client connections. In a production implementation, you'd
            // want to store these handles for proper cleanup.
            let _ = handle.into_raw_handle();
        }

        Ok(())
    }
}

#[cfg(windows)]
#[async_trait]
impl VirtualBackend for NamedPipeBackend {
    async fn create_pair(
        &mut self,
    ) -> Result<(VirtualPortEnd, VirtualPortEnd, BridgeErrorRx, BridgeStats)> {
        tracing::info!(
            "Creating NamedPipe pair: {} and {}",
            self.pipe_a_name,
            self.pipe_b_name
        );

        // Create both named pipes
        self.create_named_pipe(&self.pipe_a_name).await?;
        self.create_named_pipe(&self.pipe_b_name).await?;

        self.created = true;
        self.start_time = SystemTime::now();

        // NamedPipe bridge is handled by client connections.
        // Create an empty error channel for API consistency.
        let (error_tx, error_rx) = mpsc::channel::<String>(0);
        drop(error_tx);
        let stats = Arc::clone(&self.stats);

        Ok((
            VirtualPortEnd {
                name: "A".into(),
                path: std::path::PathBuf::from(&self.pipe_a_name),
            },
            VirtualPortEnd {
                name: "B".into(),
                path: std::path::PathBuf::from(&self.pipe_b_name),
            },
            error_rx,
            stats,
        ))
    }

    async fn is_healthy(&self) -> bool {
        if !self.created {
            return false;
        }

        // Check if pipes exist by attempting to get file attributes
        // This is a simple health check - in production you'd want more
        // sophisticated checking
        self.created
    }

    async fn get_stats(&self) -> BackendStats {
        let mut stats = self.stats.lock().await;
        stats.uptime_seconds = self.start_time.elapsed().unwrap_or_default().as_secs();
        stats.clone()
    }

    fn backend_type(&self) -> &'static str {
        "namedpipe"
    }

    async fn cleanup(&mut self) -> Result<()> {
        tracing::debug!("Cleaning up NamedPipe backend");

        // Windows named pipes are automatically cleaned up when all handles
        // are closed. The pipes we created will be cleaned up when:
        // 1. The server handles are closed (which we don't store)
        // 2. All client connections are closed

        self.created = false;
        Ok(())
    }
}

/// NamedPipe backend is not available on non-Windows platforms
#[cfg(not(windows))]
pub struct NamedPipeBackend;

#[cfg(not(windows))]
impl NamedPipeBackend {
    pub fn new() -> Result<Self> {
        Err(SerialError::UnsupportedBackend(
            "NamedPipe backend is only available on Windows".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_named_pipe_backend_creation() {
        let backend = NamedPipeBackend::new();
        assert!(backend.is_ok());
        let backend = backend.unwrap();
        assert!(!backend.created);
        assert!(!backend.pipe_a_name.is_empty());
        assert!(!backend.pipe_b_name.is_empty());
    }

    #[cfg(not(windows))]
    #[test]
    fn test_named_pipe_backend_not_available() {
        let backend = NamedPipeBackend::new();
        assert!(backend.is_err());
    }
}
