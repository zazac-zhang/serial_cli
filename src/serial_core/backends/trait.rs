//! Virtual backend trait definition
//!
//! This module defines the core trait that all virtual backends must implement.

use crate::error::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

/// Represents a single end of a virtual serial pair
#[derive(Debug, Clone)]
pub struct VirtualPortEnd {
    /// Human-readable name (e.g., "A", "B")
    pub name: String,
    /// Path to the port device (e.g., /dev/pts/0, \\.\pipe\serial_cli_a_123)
    pub path: PathBuf,
}

/// Channel for bridge errors (shared between backend and VirtualSerialPair)
pub type BridgeErrorRx = mpsc::Receiver<String>;

/// Shared backend stats
pub type BridgeStats = Arc<Mutex<BackendStats>>;

/// Runtime statistics for a virtual port backend
#[derive(Debug, Clone, Default)]
pub struct BackendStats {
    /// Total bytes read
    pub bytes_read: u64,
    /// Total bytes written
    pub bytes_written: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// Core trait that all virtual backends must implement
#[async_trait]
pub trait VirtualBackend: Send + Sync {
    /// Create a virtual serial pair and start the data bridge.
    ///
    /// Returns (port_a, port_b, error_rx, stats) where error_rx carries bridge
    /// errors and stats is a shared Arc<Mutex> updated by the bridge task.
    async fn create_pair(
        &mut self,
    ) -> Result<(VirtualPortEnd, VirtualPortEnd, BridgeErrorRx, BridgeStats)>;

    /// Check if the backend is healthy/running
    async fn is_healthy(&self) -> bool;

    /// Get runtime statistics
    async fn get_stats(&self) -> BackendStats;

    /// Get backend type identifier
    fn backend_type(&self) -> &'static str;

    /// Clean up resources (stop bridge, wait, release handles)
    async fn cleanup(&mut self) -> Result<()>;
}
