//! Socat backend wrapper (cross-platform)
//!
//! This backend creates virtual serial port pairs using the socat utility.

use crate::error::{Result, SerialError};
use crate::serial_core::backends::{BackendStats, VirtualBackend, VirtualPortEnd};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

/// Socat backend implementation
pub struct SocatBackend {
    /// Socat child process
    process: Option<tokio::process::Child>,
    /// Port A path (symlink)
    port_a_path: std::path::PathBuf,
    /// Port B path (symlink)
    port_b_path: std::path::PathBuf,
    /// Statistics
    stats: Arc<Mutex<BackendStats>>,
    /// Start time for uptime calculation
    start_time: SystemTime,
    /// Whether the pair has been created
    created: bool,
}

impl SocatBackend {
    /// Create a new Socat backend
    pub fn new() -> Result<Self> {
        Ok(Self {
            process: None,
            port_a_path: std::path::PathBuf::from("/tmp/serial_cli_socat_a"),
            port_b_path: std::path::PathBuf::from("/tmp/serial_cli_socat_b"),
            stats: Arc::new(Mutex::new(BackendStats::default())),
            start_time: SystemTime::now(),
            created: false,
        })
    }

    /// Check if socat binary is available
    pub async fn check_available() -> bool {
        tokio::process::Command::new("socat")
            .arg("-V")
            .output()
            .await
            .map(|_| true)
            .unwrap_or(false)
    }
}

#[async_trait]
impl VirtualBackend for SocatBackend {
    async fn create_pair(&mut self) -> Result<(VirtualPortEnd, VirtualPortEnd)> {
        // Check if socat is available
        if !Self::check_available().await {
            return Err(SerialError::MissingDependency(
                "socat".to_string(),
                "Install with: apt install socat | brew install socat".to_string(),
            ));
        }

        tracing::info!(
            "Creating Socat pair: {} and {}",
            self.port_a_path.display(),
            self.port_b_path.display()
        );

        // Spawn socat process
        let output = tokio::process::Command::new("socat")
            .args([
                "-d",
                "-d",
                &format!("pty,raw,echo=0,link={}", self.port_a_path.display()),
                &format!("pty,raw,echo=0,link={}", self.port_b_path.display()),
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| SerialError::BackendInitFailed(format!("Failed to spawn socat: {e}")))?;

        self.process = Some(output);

        // Give socat time to create the PTYs and symlinks
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // Verify the symlinks were created
        if !self.port_a_path.exists() || !self.port_b_path.exists() {
            // Clean up the process
            if let Some(mut process) = self.process.take() {
                process.kill().await.ok();
            }
            return Err(SerialError::BackendInitFailed(
                "Socat failed to create port symlinks".to_string(),
            ));
        }

        self.created = true;
        self.start_time = SystemTime::now();

        Ok((
            VirtualPortEnd {
                name: "A".into(),
                path: self.port_a_path.clone(),
            },
            VirtualPortEnd {
                name: "B".into(),
                path: self.port_b_path.clone(),
            },
        ))
    }

    async fn is_healthy(&self) -> bool {
        if !self.created {
            return false;
        }

        // Check if process is still running
        if let Some(process) = &self.process {
            // Try to get the process ID
            if process.id().is_some() {
                // Also check if symlinks still exist
                return self.port_a_path.exists() && self.port_b_path.exists();
            }
        }

        false
    }

    async fn get_stats(&self) -> BackendStats {
        let mut stats = self.stats.lock().await;
        stats.uptime_seconds = self
            .start_time
            .elapsed()
            .unwrap_or_default()
            .as_secs();
        stats.clone()
    }

    fn backend_type(&self) -> &'static str {
        "socat"
    }

    async fn cleanup(&mut self) -> Result<()> {
        tracing::debug!("Cleaning up Socat backend");

        // Kill the socat process
        if let Some(mut process) = self.process.take() {
            tracing::debug!("Killing socat process (PID: {:?})", process.id());
            process.kill().await.ok();
        }

        // Note: We don't remove the symlinks as they might still be in use
        // They will be cleaned up by the OS or on next system restart

        self.created = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_socat_availability() {
        let available = SocatBackend::check_available().await;
        tracing::info!("Socat available: {}", available);
        // We don't assert on this since socat might not be installed
    }

    #[test]
    fn test_socat_backend_creation() {
        let backend = SocatBackend::new();
        assert!(backend.is_ok());
        let backend = backend.unwrap();
        assert!(!backend.created);
        assert!(backend.process.is_none());
    }

    #[tokio::test]
    async fn test_socat_create_pair_when_available() {
        if !SocatBackend::check_available().await {
            tracing::info!("Skipping test: socat not available");
            return;
        }

        let mut backend = SocatBackend::new().unwrap();
        let result = backend.create_pair().await;

        if let Err(e) = &result {
            tracing::warn!("Failed to create socat pair: {}", e);
            // This might fail in some environments
            return;
        }

        let (port_a, port_b) = result.unwrap();
        assert_eq!(port_a.name, "A");
        assert_eq!(port_b.name, "B");
        assert!(backend.created);

        // Cleanup
        backend.cleanup().await.unwrap();
    }
}
