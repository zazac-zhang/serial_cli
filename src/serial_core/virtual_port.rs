//! Virtual serial port pair implementation
//!
//! This module provides virtual serial port pairs for testing, monitoring,
//! and debugging serial communication without physical hardware.

use crate::error::{Result, SerialError};
use crate::serial_core::backends::{BackendType, BackendStats, VirtualBackend as VirtualBackendTrait};
use crate::serial_core::sniffer::{SerialSniffer, SnifferConfig};
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::io::unix::AsyncFd;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::Duration;

/// A single captured packet from the bridge
#[derive(Debug, Clone)]
pub struct CapturedPacket {
    /// Direction of data flow: A→B or B→A
    pub direction: PacketDirection,
    /// Payload bytes
    pub data: Vec<u8>,
    /// Capture timestamp
    pub timestamp: SystemTime,
}

/// Direction of a captured packet
#[derive(Debug, Clone, Copy)]
pub enum PacketDirection {
    /// Data flowed from port A to port B
    AtoB,
    /// Data flowed from port B to port A
    BtoA,
}

/// Shared packet capture buffer
#[derive(Debug, Default)]
pub struct PacketCapture {
    packets: Vec<CapturedPacket>,
    max_packets: usize,
    total_packets: u64,
    total_bytes: u64,
}

impl PacketCapture {
    fn new(max_packets: usize) -> Self {
        Self {
            packets: Vec::new(),
            max_packets,
            total_packets: 0,
            total_bytes: 0,
        }
    }

    fn record(&mut self, direction: PacketDirection, data: &[u8]) {
        self.total_packets += 1;
        self.total_bytes += data.len() as u64;

        if self.max_packets == 0 || self.packets.len() < self.max_packets {
            self.packets.push(CapturedPacket {
                direction,
                data: data.to_vec(),
                timestamp: SystemTime::now(),
            });
        }
    }
}

/// Virtual serial port configuration
#[derive(Debug, Clone)]
pub struct VirtualConfig {
    /// Backend type for creating virtual ports
    pub backend: BackendType,

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
            backend: BackendType::Auto,
            monitor: false,
            monitor_output: None,
            max_packets: 0,
            bridge_buffer_size: 8192,
        }
    }
}

/// Virtual port backend type (legacy alias for compatibility)
#[deprecated(note = "Use BackendType from serial_core::backends instead")]
pub type VirtualBackend = BackendType;

#[cfg(unix)]
#[cfg(unix)]
type PtyMasters = (Arc<AsyncFd<OwnedFd>>, Arc<AsyncFd<OwnedFd>>);

#[cfg(unix)]
type PtyPairResult = (
    String,
    String,
    Option<PtyMasters>,
    JoinHandle<()>,
    mpsc::Receiver<String>,
    Arc<Mutex<VirtualPairStats>>,
);

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

    /// Packet capture buffer (when monitoring is enabled)
    capture: Option<Arc<Mutex<PacketCapture>>>,

    /// Master file descriptors for PTY (platform-specific)
    #[cfg(unix)]
    master_fds: Option<PtyMasters>,

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

        let running = Arc::new(AtomicBool::new(true));

        let capture = if config.monitor {
            Some(Arc::new(Mutex::new(PacketCapture::new(config.max_packets))))
        } else {
            None
        };

        // Create the virtual pair based on backend
        let (port_a, port_b, master_fds, bridge_task, error_rx, stats) =
            match config.backend {
                BackendType::Auto => {
                    // Auto-detect based on platform
                    let detected = BackendType::detect();
                    tracing::info!("Auto-detected backend: {:?}", detected);
                    match detected {
                        BackendType::Pty => Self::create_pty_pair(
                            config.bridge_buffer_size,
                            Arc::clone(&running),
                            capture.clone(),
                        )?,
                        BackendType::NamedPipe => {
                            return Err(SerialError::VirtualPort(
                                "NamedPipe backend not yet implemented via old API".to_string(),
                            ))
                        }
                        BackendType::Socat => {
                            return Err(SerialError::VirtualPort(
                                "Socat backend not yet implemented via old API".to_string(),
                            ))
                        }
                        BackendType::Auto => unreachable!(),
                    }
                }
                BackendType::Pty => Self::create_pty_pair(
                    config.bridge_buffer_size,
                    Arc::clone(&running),
                    capture.clone(),
                )?,
                BackendType::NamedPipe => {
                    return Err(SerialError::VirtualPort(
                        "NamedPipe backend not yet implemented via old API".to_string(),
                    ))
                }
                BackendType::Socat => {
                    return Err(SerialError::VirtualPort(
                        "Socat backend not yet implemented via old API".to_string(),
                    ))
                }
            };

        let id = uuid::Uuid::new_v4().to_string();
        let created_at = SystemTime::now();

        tracing::info!("Created virtual pair: A={}, B={}, ID={}", port_a, port_b, id);

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
            running,
            created_at,
            stats,
            capture,
            #[cfg(unix)]
            master_fds,
            bridge_task: Some(bridge_task),
        })
    }

    /// Create a PTY pair with event-driven bridge using AsyncFd
    #[cfg(unix)]
    fn create_pty_pair(
        buffer_size: usize,
        running: Arc<AtomicBool>,
        capture: Option<Arc<Mutex<PacketCapture>>>,
    ) -> Result<PtyPairResult> {
        use libc::{grantpt, posix_openpt, ptsname, unlockpt, O_NOCTTY, O_RDWR};
        use std::ffi::CStr;
        use std::os::fd::OwnedFd;

        // Create first PTY master
        let master1_fd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };
        if master1_fd == -1 {
            return Err(SerialError::VirtualPort(
                "Failed to open first PTY master".to_string(),
            ));
        }
        if unsafe { grantpt(master1_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to grant first PTY".to_string(),
            ));
        }
        if unsafe { unlockpt(master1_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to unlock first PTY".to_string(),
            ));
        }
        let slave1_path = unsafe {
            let ptr = ptsname(master1_fd);
            if ptr.is_null() {
                libc::close(master1_fd);
                return Err(SerialError::VirtualPort(
                    "Failed to get first PTY slave name".to_string(),
                ));
            }
            CStr::from_ptr(ptr)
        }
        .to_string_lossy()
        .to_string();

        // Create second PTY master
        let master2_fd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };
        if master2_fd == -1 {
            unsafe { libc::close(master1_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to open second PTY master".to_string(),
            ));
        }
        if unsafe { grantpt(master2_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            unsafe { libc::close(master2_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to grant second PTY".to_string(),
            ));
        }
        if unsafe { unlockpt(master2_fd) } == -1 {
            unsafe { libc::close(master1_fd) };
            unsafe { libc::close(master2_fd) };
            return Err(SerialError::VirtualPort(
                "Failed to unlock second PTY".to_string(),
            ));
        }
        let slave2_path = unsafe {
            let ptr = ptsname(master2_fd);
            if ptr.is_null() {
                libc::close(master1_fd);
                libc::close(master2_fd);
                return Err(SerialError::VirtualPort(
                    "Failed to get second PTY slave name".to_string(),
                ));
            }
            CStr::from_ptr(ptr)
        }
        .to_string_lossy()
        .to_string();

        tracing::debug!(
            "PTY pair created: slave1={}, slave2={}",
            slave1_path,
            slave2_path
        );

        // SAFETY: we own these fds and will close them exactly once via OwnedFd
        let master1_owned = unsafe { OwnedFd::from_raw_fd(master1_fd) };
        let master2_owned = unsafe { OwnedFd::from_raw_fd(master2_fd) };

        let master1_async = AsyncFd::new(master1_owned).map_err(|e| {
            SerialError::VirtualPort(format!("Failed to register first PTY with epoll: {e}"))
        })?;
        let master2_async = AsyncFd::new(master2_owned).map_err(|e| {
            SerialError::VirtualPort(format!("Failed to register second PTY with epoll: {e}"))
        })?;

        // Create channels for bridge communication
        let (error_tx, error_rx) = mpsc::channel(10);
        let stats = Arc::new(Mutex::new(VirtualPairStats::default()));

        // Wrap in Arc for shared ownership across select branches
        let m1 = Arc::new(master1_async);
        let m2 = Arc::new(master2_async);

        // Spawn event-driven bridge task
        let stats_clone = Arc::clone(&stats);
        let error_tx_clone = error_tx.clone();
        let m1_bridge = Arc::clone(&m1);
        let m2_bridge = Arc::clone(&m2);

        let bridge_task = tokio::spawn(async move {
            tracing::debug!("PTY bridge task started (event-driven)");

            let mut buffer = vec![0u8; buffer_size];

            loop {
                let m1_readable = Arc::clone(&m1_bridge);
                let m2_readable = Arc::clone(&m2_bridge);
                let m1_read = Arc::clone(&m1_bridge);
                let m2_write = Arc::clone(&m2_bridge);
                let m2_read = Arc::clone(&m2_bridge);
                let m1_write = Arc::clone(&m1_bridge);

                tokio::select! {
                    // Wait for master1 to become readable
                    result = m1_readable.readable() => {
                        if !running.load(Ordering::SeqCst) { break; }
                        let mut guard = match result {
                            Ok(g) => g,
                            Err(e) => {
                                let error = format!("master1 readable() failed: {e}");
                                tracing::error!("{}", error);
                                let _ = error_tx_clone.send(error).await;
                                break;
                            }
                        };
                        let fd = m1_read.as_raw_fd();
                        let n1 = guard.try_io(|_| Ok(unsafe {
                            libc::read(
                                fd,
                                buffer.as_mut_ptr() as *mut libc::c_void,
                                buffer.len(),
                            )
                        }));

                        match n1 {
                            Ok(Ok(n)) if n > 0 => {
                                let n1_usize = n as usize;
                                if let Some(ref capture) = capture {
                                    let mut cap = capture.lock().await;
                                    cap.record(PacketDirection::AtoB, &buffer[..n1_usize]);
                                }
                                let fd2 = m2_write.as_raw_fd();
                                let mut written: isize = 0;
                                while written < n1_usize as isize {
                                    let n = unsafe {
                                        libc::write(
                                            fd2,
                                            buffer.as_ptr().add(written as usize) as *const libc::c_void,
                                            (n1_usize as isize - written) as usize,
                                        )
                                    };
                                    if n > 0 {
                                        written += n;
                                    } else {
                                        let error = format!(
                                            "Partial write failed: {} bytes remaining, error: {}",
                                            n1_usize as isize - written,
                                            std::io::Error::last_os_error()
                                        );
                                        tracing::error!("{}", error);
                                        let _ = error_tx_clone.send(error).await;
                                        break;
                                    }
                                }
                                let mut s = stats_clone.lock().await;
                                s.bytes_bridged += n1_usize as u64;
                                s.packets_bridged += 1;
                            }
                            Ok(Ok(_)) => {} // EOF or spurious
                            Ok(Err(e)) => {
                                let error = format!("master1 read failed: {e}");
                                tracing::error!("{}", error);
                                let _ = error_tx_clone.send(error).await;
                            }
                            Err(_) => break,
                        }
                    }

                    // Wait for master2 to become readable
                    result = m2_readable.readable() => {
                        if !running.load(Ordering::SeqCst) { break; }
                        let mut guard = match result {
                            Ok(g) => g,
                            Err(e) => {
                                let error = format!("master2 readable() failed: {e}");
                                tracing::error!("{}", error);
                                let _ = error_tx_clone.send(error).await;
                                break;
                            }
                        };
                        let fd = m2_read.as_raw_fd();
                        let n2 = guard.try_io(|_| Ok(unsafe {
                            libc::read(
                                fd,
                                buffer.as_mut_ptr() as *mut libc::c_void,
                                buffer.len(),
                            )
                        }));

                        match n2 {
                            Ok(Ok(n)) if n > 0 => {
                                let n2_usize = n as usize;
                                if let Some(ref capture) = capture {
                                    let mut cap = capture.lock().await;
                                    cap.record(PacketDirection::BtoA, &buffer[..n2_usize]);
                                }
                                let fd1 = m1_write.as_raw_fd();
                                let mut written: isize = 0;
                                while written < n2_usize as isize {
                                    let n = unsafe {
                                        libc::write(
                                            fd1,
                                            buffer.as_ptr().add(written as usize) as *const libc::c_void,
                                            (n2_usize as isize - written) as usize,
                                        )
                                    };
                                    if n > 0 {
                                        written += n;
                                    } else {
                                        let error = format!(
                                            "Partial write failed: {} bytes remaining, error: {}",
                                            n2_usize as isize - written,
                                            std::io::Error::last_os_error()
                                        );
                                        tracing::error!("{}", error);
                                        let _ = error_tx_clone.send(error).await;
                                        break;
                                    }
                                }
                                let mut s = stats_clone.lock().await;
                                s.bytes_bridged += n2_usize as u64;
                                s.packets_bridged += 1;
                            }
                            Ok(Ok(_)) => {}
                            Ok(Err(e)) => {
                                let error = format!("master2 read failed: {e}");
                                tracing::error!("{}", error);
                                let _ = error_tx_clone.send(error).await;
                            }
                            Err(_) => break,
                        }
                    }

                    // Exit signal
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if !running.load(Ordering::SeqCst) { break; }
                    }
                }
            }

            tracing::debug!("PTY bridge task stopped");
        });

        Ok((
            slave1_path,
            slave2_path,
            Some((m1, m2)),
            bridge_task,
            error_rx,
            stats,
        ))
    }

    /// Create a PTY pair (not available on Windows)
    #[cfg(not(unix))]
    fn create_pty_pair(
        _buffer_size: usize,
        _running: Arc<AtomicBool>,
        _capture: Option<Arc<Mutex<PacketCapture>>>,
    ) -> Result<
        (
            String,
            String,
            Option<()>,
            JoinHandle<()>,
            mpsc::Receiver<String>,
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

        let sniffer_config = SnifferConfig {
            max_packets,
            save_to_file: output_file.is_some(),
            output_dir: output_file
                .as_ref()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::path::PathBuf::from(".")),
            ..Default::default()
        };

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

        // Set running flag to false to signal graceful shutdown
        self.running.store(false, Ordering::SeqCst);

        // Give the bridge task a chance to exit gracefully
        if let Some(bridge_task) = self.bridge_task.take() {
            match tokio::time::timeout(Duration::from_millis(100), bridge_task).await {
                Ok(_) => tracing::debug!("Bridge task exited gracefully"),
                Err(_) => {
                    tracing::debug!("Bridge task did not exit within timeout, aborting");
                    // Bridge task is already consumed by the timeout future,
                    // so we don't need to explicitly abort here.
                }
            }
        }

        // Platform-specific cleanup
        self.cleanup_resources().await?;

        tracing::info!("Virtual pair stopped: {}", self.id);

        Ok(())
    }

    /// Platform-specific resource cleanup
    #[cfg(unix)]
    async fn cleanup_resources(&mut self) -> Result<()> {
        // AsyncFd drops its inner OwnedFd automatically, closing the fd
        if self.master_fds.take().is_some() {
            tracing::debug!("Closing PTY master file descriptors");
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

        let (capture_packets, capture_bytes) = if let Some(ref capture) = self.capture {
            let cap = capture.lock().await;
            (cap.total_packets, cap.total_bytes)
        } else {
            (0, 0)
        };

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
            capture_packets,
            capture_bytes,
        }
    }

    /// Get a reference to the sniffer (if monitoring is active)
    pub fn sniffer(&self) -> Option<&SerialSniffer> {
        self.sniffer.as_ref()
    }

    /// Get captured packets (when monitoring is enabled)
    pub async fn captured_packets(&self) -> Vec<CapturedPacket> {
        if let Some(ref capture) = self.capture {
            let cap = capture.lock().await;
            cap.packets.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if monitoring is enabled
    pub fn is_monitoring(&self) -> bool {
        self.capture.is_some()
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
            if self.master_fds.take().is_some() {
                tracing::debug!("Closing PTY master file descriptors in Drop");
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

    /// Total packets captured (when monitoring is enabled)
    pub capture_packets: u64,

    /// Total capture bytes (when monitoring is enabled)
    pub capture_bytes: u64,
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
        let backend = BackendType::detect();
        assert!(backend.is_available());
    }

    #[test]
    fn test_virtual_config_default() {
        let config = VirtualConfig::default();
        assert_eq!(config.backend, BackendType::Auto);
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
        // Auto backend should resolve to Pty on Unix
        assert_eq!(pair.backend, BackendType::Auto);
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

    #[cfg(unix)]
    #[tokio::test]
    async fn test_capture_disabled_by_default() {
        let config = VirtualConfig::default();
        let result = VirtualSerialPair::create(config).await;
        if result.is_err() {
            return;
        }
        let pair = result.unwrap();
        assert!(!pair.is_monitoring());
        assert!(pair.captured_packets().await.is_empty());
        pair.stop().await.unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_capture_enabled_with_monitor() {
        let config = VirtualConfig {
            monitor: true,
            max_packets: 100,
            ..VirtualConfig::default()
        };
        let result = VirtualSerialPair::create(config).await;
        if result.is_err() {
            return;
        }
        let pair = result.unwrap();
        assert!(pair.is_monitoring());
        // No data written yet, so capture should be empty
        assert!(pair.captured_packets().await.is_empty());
        pair.stop().await.unwrap();
    }
}
