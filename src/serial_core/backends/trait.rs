//! Virtual backend trait definition
//!
//! This module defines the core trait that all virtual backends must implement.

use crate::error::Result;
use async_trait::async_trait;
use std::path::PathBuf;

/// Represents a single end of a virtual serial pair
#[derive(Debug, Clone)]
pub struct VirtualPortEnd {
    /// Human-readable name (e.g., "A", "B")
    pub name: String,
    /// Path to the port device (e.g., /dev/pts/0, \\.\pipe\serial_cli_a_123)
    pub path: PathBuf,
}

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
    /// Create a virtual serial pair
    ///
    /// Returns two `VirtualPortEnd` structs representing the two ends of the pair.
    async fn create_pair(&mut self) -> Result<(VirtualPortEnd, VirtualPortEnd)>;

    /// Check if the backend is healthy/running
    ///
    /// Returns true if the backend is operational, false otherwise.
    async fn is_healthy(&self) -> bool;

    /// Get runtime statistics
    ///
    /// Returns current statistics for the backend.
    async fn get_stats(&self) -> BackendStats;

    /// Get backend type identifier
    ///
    /// Returns a string identifier for the backend type.
    fn backend_type(&self) -> &'static str;

    /// Clean up resources
    ///
    /// Called when the virtual port pair is being destroyed.
    async fn cleanup(&mut self) -> Result<()>;
}
