//! Virtual serial port pair implementation
//!
//! This module provides virtual serial port pairs for testing, monitoring,
//! and debugging serial communication without physical hardware.

use crate::error::{Result, SerialError};
use crate::serial_core::sniffer::{SerialSniffer, SnifferConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

/// Virtual serial port configuration
#[derive(Debug, Clone)]
pub struct VirtualConfig {
    /// Backend type for creating virtual ports
    pub backend: VirtualBackend,

    /// Enable traffic monitoring
    pub monitor: bool,

    /// Monitoring output file (optional)
    pub monitor_output: Option<std::path::PathBuf>,

    /// Maximum packets to capture (0 = unlimited)
    pub max_packets: usize,

    /// Bridge buffer size
    pub bridge_buffer_size: usize,
}

impl Default for VirtualConfig {
    fn default() -> Self {
        Self {
            backend: VirtualBackend::Pty,
            monitor: false,
            monitor_output: None,
            max_packets: 0,
            bridge_buffer_size: 8192,
        }
    }
}

/// Virtual port backend type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualBackend {
    /// POSIX PTY (pseudo-terminal) - Unix/Linux/macOS
    Pty,

    /// Windows Named Pipes - Windows only
    NamedPipe,

    /// External socat process - Cross-platform
    Socat,
}

impl VirtualBackend {
    /// Get the default backend for the current platform
    pub fn default_for_platform() -> Self {
        #[cfg(unix)]
        return VirtualBackend::Pty;

        #[cfg(windows)]
        return VirtualBackend::NamedPipe;
    }

    /// Check if this backend is available on the current platform
    pub fn is_available(&self) -> bool {
        match self {
            VirtualBackend::Pty => {
                #[cfg(unix)]
                return true;

                #[cfg(not(unix))]
                return false;
            }
            VirtualBackend::NamedPipe => {
                #[cfg(windows)]
                return true;

                #[cfg(not(windows))]
                return false;
            }
            VirtualBackend::Socat => true, // Available everywhere if installed
        }
    }
}

/// Virtual serial port pair
pub struct VirtualSerialPair {
    /// Unique identifier for this pair
    pub id: String,

    /// Port A name (e.g., /dev/pts/0)
    pub port_a: String,

    /// Port B name (e.g., /dev/pts/1)
    pub port_b: String,

    /// Backend type used
    pub backend: VirtualBackend,

    /// Sniffer for monitoring traffic
    sniffer: Option<SerialSniffer>,

    /// Running state
    running: Arc<AtomicBool>,

    /// Creation timestamp
    pub created_at: SystemTime,

    /// Statistics
    stats: Arc<Mutex<VirtualPairStats>>,

    /// Master file descriptors for PTY (platform-specific)
    #[cfg(unix)]
    master_fds: Option<(std::os::fd::RawFd, std::os::fd::RawFd)>,

    /// Bridge task handle
    bridge_task: Option<JoinHandle<()>>,
}

/// Virtual pair statistics
#[derive(Debug, Clone, Default)]
pub struct VirtualPairStats {
    pub bytes_bridged: u64,
    pub packets_bridged: u64,
    pub bridge_errors: u64,
    pub last_error: Option<String>,
}

impl VirtualSerialPair {
    /// Create a new virtual serial port pair
    pub async fn create(config: VirtualConfig) -> Result<Self> {
        // Check if backend is available
        if !config.backend.is_available() {
            return Err(SerialError::VirtualPort(format!(
                "Backend {:?} is not available on this platform",
                config.backend
            )));
        }

        tracing::info!("Creating virtual serial port pair with backend: {:?}", config.backend);

        // Create the virtual pair based on backend
        let (port_a, port_b, master_fds, bridge_task, error_tx, stats) =
            match config.backend {
                VirtualBackend::Pty => Self::create_pty_pair(
                    config.bridge_buffer_size,
                    config.monitor,
                    config.max_packets,
                )?,
                VirtualBackend::NamedPipe => {
                    return Err(SerialError::VirtualPort(
                        "NamedPipe backend not yet implemented".to_string(),
                    ))
                }
                VirtualBackend::Socat => {
                    return Err(SerialError::VirtualPort(
                        "Socat backend not yet implemented".to_string(),
                    ))
                }
            };

        let id = uuid::Uuid::new_v4().to_string();
        let created_at = SystemTime::now();

        tracing::info!("Created virtual pair: A={}, B={}, ID={}", port_a, port_b, id);

        let (error_tx, error_rx) = mpsc::channel(10);

        // Spawn error monitoring task
        let stats_clone = Arc::clone(&stats);
        tokio::spawn(async move {
            let mut error_rx = error_rx;
            while let Some(error) = error_rx.recv().await {
                tracing::error!("Bridge error: {}", error);
                let mut stats = stats_clone.lock().await;
                stats.bridge_errors += 1;
                stats.last_error = Some(error);
            }
        });

        Ok(Self {
            id: id.clone(),
            port_a,
            port_b,
            backend: config.backend,
            sniffer: None,
            running: Arc::new(AtomicBool::new(true)),
            created_at,
            stats,
            #[cfg(unix)]
            master_fds,
            bridge_task: Some(bridge_task),
        })
    }

    /// Create a PTY pair with improved error handling and macOS support
    #[cfg(unix)]
    fn create_pty_pair(
        buffer_size: usize,
        _monitor: bool,
        _max_packets: usize,
    ) -> Result<(
        String,
        String,
        Option<(std::os::fd::RawFd, std::os::fd::RawFd)>,
        JoinHandle<()>,
        mpsc::Sender<String>,
        Arc<Mutex<VirtualPairStats>>,
    )> {
        use libc::{grantpt, posix_openpt, ptsname, unlockpt, O_NOCTTY, O_RDWR};
        use std::ffi::CStr;
        use std::os::fd::RawFd;

        // Create first PTY master
        //
        // SAFETY: posix_openpt is a libc function that returns a file descriptor or -1 on error.
        // We check for -1 return value and handle errors appropriately.
        let master1_fd: RawFd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };

        if master1_fd == -1 {
            return Err(SerialError::VirtualPort(
                "Failed to open first PTY master".to_string(),
            ));
        }

        // Grant slave PTY
        if unsafe { grantpt(master1_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to grant first PTY".to_string(),
            ));
        }

        // Unlock slave PTY
        if unsafe { unlockpt(master1_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to unlock first PTY".to_string(),
            ));
        }

        // Get first slave PTY name
        let slave1_name = unsafe {
            let ptr = ptsname(master1_fd);
            if ptr.is_null() {
                libc::close(master1_fd);
                return Err(SerialError::VirtualPort(
                    "Failed to get first PTY slave name".to_string(),
                ));
            }
            CStr::from_ptr(ptr)
        };

        let slave1_path = slave1_name.to_string_lossy().to_string();

        // Create second PTY master
        let master2_fd: RawFd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };

        if master2_fd == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to open second PTY master".to_string(),
            ));
        }

        // Grant second slave PTY
        if unsafe { grantpt(master2_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            unsafe { libc::close(master2_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to grant second PTY".to_string(),
            ));
        }

        // Unlock second slave PTY
        if unsafe { unlockpt(master2_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            unsafe { libc::close(master2_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to unlock second PTY".to_string(),
            ));
        }

        // Get second slave PTY name
        let slave2_name = unsafe {
            let ptr = ptsname(master2_fd);
            if ptr.is_null() {
                libc::close(master1_fd);
                libc::close(master2_fd);
                return Err(SerialError::VirtualPort(
                    "Failed to get second PTY slave name".to_string(),
                ));
            }
            CStr::from_ptr(ptr)
        };

        let slave2_path = slave2_name.to_string_lossy().to_string();

        tracing::debug!(
            "PTY pair created: slave1={}, slave2={}",
            slave1_path,
            slave2_path
        );

        // Set non-blocking mode for both masters (CRITICAL for all Unix including macOS)
        use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};

        for (fd, name) in [
            (master1_fd, "first PTY"),
            (master2_fd, "second PTY"),
        ] {
            let flags = unsafe { fcntl(fd, F_GETFL, 0) };
            if flags == -1 {
                unsafe {
                    libc::close(master1_fd);
                    libc::close(master2_fd);
                };
                return Err(SerialError::VirtualPort(format!(
                    "Failed to get {} flags",
                    name
                )));
            }

            if unsafe { fcntl(fd, F_SETFL, flags | O_NONBLOCK) } == -1 {
                unsafe {
                    libc::close(master1_fd);
                    libc::close(master2_fd);
                };
                return Err(SerialError::VirtualPort(format!(
                    "Failed to set {} non-blocking",
                    name
                )));
            }

            tracing::debug!("Set {} to non-blocking mode", name);
        }

        // Create channels for bridge communication
        let (error_tx, _error_rx) = mpsc::channel(10);
        let stats = Arc::new(Mutex::new(VirtualPairStats::default()));

        // Spawn improved bridge task with proper error handling
        let stats_clone = Arc::clone(&stats);
        let error_tx_clone = error_tx.clone();

        let bridge_task = tokio::spawn(async move {
            tracing::debug!("PTY bridge task started");

            let mut buffer = vec![0u8; buffer_size];

            loop {
                // Small sleep to prevent busy-waiting but much shorter than before
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

                // Try to read from master1 and write to master2
                let n1 = unsafe {
                    libc::read(
                        master1_fd,
                        buffer.as_mut_ptr() as *mut libc::c_void,
                        buffer.len(),
                    )
                };

                if n1 > 0 {
                    let mut written: isize = 0;
                    while written < n1 {
                        let n = unsafe {
                            libc::write(
                                master2_fd,
                                buffer.as_ptr().add(written as usize) as *const libc::c_void,
                                (n1 - written) as usize,
                            )
                        };

                        if n > 0 {
                            written += n;
                        } else {
                            // Handle write error properly
                            let error = format!(
                                "Partial write failed: {} bytes remaining, error: {}",
                                n1 - written,
                                std::io::Error::last_os_error()
                            );
                            tracing::error!("{}", error);
                            let _ = error_tx_clone.send(error).await;
                            break;
                        }
                    }

                    // Update statistics
                    let mut stats = stats_clone.lock().await;
                    stats.bytes_bridged += n1 as u64;
                    stats.packets_bridged += 1;
                }

                // Try to read from master2 and write to master1
                let n2 = unsafe {
                    libc::read(
                        master2_fd,
                        buffer.as_mut_ptr() as *mut libc::c_void,
                        buffer.len(),
                    )
                };

                if n2 > 0 {
                    let mut written: isize = 0;
                    while written < n2 {
                        let n = unsafe {
                            libc::write(
                                master1_fd,
                                buffer.as_ptr().add(written as usize) as *const libc::c_void,
                                (n2 - written) as usize,
                            )
                        };

                        if n > 0 {
                            written += n;
                        } else {
                            // Handle write error properly
                            let error = format!(
                                "Partial write failed: {} bytes remaining, error: {}",
                                n2 - written,
                                std::io::Error::last_os_error()
                            );
                            tracing::error!("{}", error);
                            let _ = error_tx_clone.send(error).await;
                            break;
                        }
                    }

                    // Update statistics
                    let mut stats = stats_clone.lock().await;
                    stats.bytes_bridged += n2 as u64;
                    stats.packets_bridged += 1;
                }
            }

            tracing::debug!("PTY bridge task stopped");
        });

        Ok((
            slave1_path,
            slave2_path,
            Some((master1_fd, master2_fd)),
            bridge_task,
            error_tx,
            stats,
        ))
    }

    /// Create a PTY pair (not available on Windows)
    #[cfg(not(unix))]
    fn create_pty_pair(
        _buffer_size: usize,
        _monitor: bool,
        _max_packets: usize,
    ) -> Result<
        (
            String,
            String,
            Option<()>,
            JoinHandle<()>,
            mpsc::Sender<String>,
            Arc<Mutex<VirtualPairStats>>,
        ),
    > {
        Err(SerialError::VirtualPort(
            "PTY backend is not available on this platform".to_string(),
        ))
    }

    /// Start monitoring traffic on this virtual pair
    pub async fn start_monitoring(&mut self) -> Result<()> {
        self.start_monitoring_with_config(None, 0).await
    }

    /// Start monitoring with specific configuration
    async fn start_monitoring_with_config(
        &mut self,
        output_file: Option<std::path::PathBuf>,
        max_packets: usize,
    ) -> Result<()> {
        tracing::info!("Starting monitoring for virtual pair: {}", self.id);

        let mut sniffer_config = SnifferConfig::default();
        sniffer_config.max_packets = max_packets;
        sniffer_config.save_to_file = output_file.is_some();

        if let Some(ref output) = output_file {
            sniffer_config.output_dir = output
                .parent()
                .unwrap_or(&std::path::PathBuf::from("."))
                .to_path_buf();
        }

        let sniffer = SerialSniffer::new(sniffer_config);

        // NOTE: Virtual port monitoring has limitations
        //
        // The bridge task forwards data between PTY masters, but we don't capture
        // this traffic for monitoring. Full monitoring would require integrating
        // packet capture into the bridge task, which would add complexity.
        //
        // For complete monitoring capabilities, use regular serial ports with
        // the sniffer functionality instead.
        //
        // This is a known limitation documented in the user guide.

        tracing::warn!(
            "Virtual port monitoring is limited. The sniffer is created but won't capture \
             actual bridge traffic. For full monitoring, use regular serial ports."
        );

        self.sniffer = Some(sniffer);
        tracing::info!("Monitoring (limited) started for virtual pair: {}", self.id);

        Ok(())
    }

    /// Stop the virtual pair and cleanup resources
    pub async fn stop(mut self) -> Result<()> {
        tracing::info!("Stopping virtual pair: {}", self.id);

        // Set running flag to false
        self.running.store(false, Ordering::SeqCst);

        // Stop bridge task if active
        if let Some(bridge_task) = self.bridge_task.take() {
            bridge_task.abort();
        }

        // Platform-specific cleanup
        self.cleanup_resources().await?;

        tracing::info!("Virtual pair stopped: {}", self.id);

        Ok(())
    }

    /// Platform-specific resource cleanup
    #[cfg(unix)]
    async fn cleanup_resources(&mut self) -> Result<()> {
        // Close the PTY master file descriptors
        if let Some((master1_fd, master2_fd)) = self.master_fds.take() {
            tracing::debug!(
                "Closing PTY master file descriptors: {}, {}",
                master1_fd,
                master2_fd
            );

            // SAFETY: close is a libc function that closes a file descriptor.
            // We own these file descriptors and are closing them exactly once.
            unsafe {
                libc::close(master1_fd);
                libc::close(master2_fd);
            }
        }
        Ok(())
    }

    #[cfg(not(unix))]
    async fn cleanup_resources(&mut self) -> Result<()> {
        tracing::debug!("No specific cleanup needed for this platform");
        Ok(())
    }

    /// Check if the pair is still running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get statistics about this virtual pair
    pub async fn stats(&self) -> VirtualStats {
        let uptime = self
            .created_at
            .elapsed()
            .unwrap_or_default()
            .as_secs();

        let bridge_stats = self.stats.lock().await.clone();

        VirtualStats {
            id: self.id.clone(),
            port_a: self.port_a.clone(),
            port_b: self.port_b.clone(),
            backend: self.backend,
            running: self.is_running(),
            uptime_secs: uptime,
            bytes_bridged: bridge_stats.bytes_bridged,
            packets_bridged: bridge_stats.packets_bridged,
            bridge_errors: bridge_stats.bridge_errors,
            last_error: bridge_stats.last_error,
        }
    }

    /// Get a reference to the sniffer (if monitoring is active)
    pub fn sniffer(&self) -> Option<&SerialSniffer> {
        self.sniffer.as_ref()
    }
}

/// Implement Drop to ensure resources are cleaned up
impl Drop for VirtualSerialPair {
    fn drop(&mut self) {
        tracing::debug!("VirtualSerialPair dropped: {}", self.id);

        // Ensure running flag is set to false
        self.running.store(false, Ordering::SeqCst);

        // Abort bridge task if still running
        if let Some(bridge_task) = self.bridge_task.take() {
            bridge_task.abort();
        }

        // Close file descriptors on Unix
        #[cfg(unix)]
        {
            if let Some((master1_fd, master2_fd)) = self.master_fds.take() {
                tracing::debug!(
                    "Closing PTY master file descriptors in Drop: {}, {}",
                    master1_fd,
                    master2_fd
                );

                // SAFETY: close is a libc function that closes a file descriptor.
                // We own these file descriptors and are closing them exactly once in the Drop impl.
                unsafe {
                    libc::close(master1_fd);
                    libc::close(master2_fd);
                }
            }
        }
    }
}

/// Virtual serial port statistics
#[derive(Debug, Clone)]
pub struct VirtualStats {
    /// Unique identifier
    pub id: String,

    /// Port A name
    pub port_a: String,

    /// Port B name
    pub port_b: String,

    /// Backend type
    pub backend: VirtualBackend,

    /// Running state
    pub running: bool,

    /// Uptime in seconds
    pub uptime_secs: u64,

    /// Total bytes bridged
    pub bytes_bridged: u64,

    /// Total packets bridged
    pub packets_bridged: u64,

    /// Number of bridge errors
    pub bridge_errors: u64,

    /// Last bridge error message
    pub last_error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_backend_availability() {
        let pty = VirtualBackend::Pty;
        assert_eq!(pty.is_available(), cfg!(unix));

        let named_pipe = VirtualBackend::NamedPipe;
        assert_eq!(named_pipe.is_available(), cfg!(windows));

        let socat = VirtualBackend::Socat;
        assert!(socat.is_available());
    }

    #[test]
    fn test_default_backend_for_platform() {
        let backend = VirtualBackend::default_for_platform();
        assert!(backend.is_available());
    }

    #[test]
    fn test_virtual_config_default() {
        let config = VirtualConfig::default();
        assert_eq!(config.backend, VirtualBackend::Pty);
        assert!(!config.monitor);
        assert!(config.monitor_output.is_none());
        assert_eq!(config.max_packets, 0);
        assert_eq!(config.bridge_buffer_size, 8192);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_create_virtual_pair() {
        let config = VirtualConfig::default();
        let result = VirtualSerialPair::create(config).await;

        if let Err(e) = &result {
            tracing::warn!("Failed to create virtual pair: {}", e);
            // This might fail in some environments (CI, containers, etc.)
            return;
        }

        let pair = result.unwrap();
        assert!(!pair.id.is_empty());
        assert!(!pair.port_a.is_empty());
        assert!(!pair.port_b.is_empty());
        assert_eq!(pair.backend, VirtualBackend::Pty);
        assert!(pair.is_running());

        // Verify file descriptors are stored
        #[cfg(unix)]
        assert!(pair.master_fds.is_some());

        // Cleanup
        pair.stop().await.unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_virtual_pair_stats() {
        let config = VirtualConfig::default();

        let result = VirtualSerialPair::create(config).await;
        if result.is_err() {
            tracing::warn!("Skipping test: virtual pair creation failed");
            return;
        }

        let pair = result.unwrap();
        let stats = pair.stats().await;

        assert_eq!(stats.id, pair.id);
        assert_eq!(stats.port_a, pair.port_a);
        assert_eq!(stats.port_b, pair.port_b);
        assert!(stats.running);

        // Cleanup
        pair.stop().await.unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_virtual_pair_cleanup() {
        let config = VirtualConfig::default();

        let result = VirtualSerialPair::create(config).await;
        if result.is_err() {
            tracing::warn!("Skipping test: virtual pair creation failed");
            return;
        }

        // Test that cleanup works when pair is dropped
        {
            let pair = result.unwrap();
            assert!(pair.is_running());
            // pair is dropped here
        }

        // Give time for cleanup
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // If we got here without panic, cleanup worked
        tracing::info!("Cleanup test passed");
    }
}
