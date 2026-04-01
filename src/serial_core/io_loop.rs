//! I/O loop for async serial operations
//!
//! This module provides the async I/O event loop for managing multiple serial ports.

use crate::error::{Result, SerialError};
use crate::serial_core::PortManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{timeout, Duration};

/// I/O event
#[derive(Debug, Clone)]
pub enum IoEvent {
    /// Data received from port
    DataReceived { port_id: String, data: Vec<u8> },
    /// Data sent to port
    DataSent { port_id: String, length: usize },
    /// Port opened
    PortOpened { port_id: String },
    /// Port closed
    PortClosed { port_id: String },
    /// Error occurred
    Error { port_id: String, error: String },
}

/// I/O loop configuration
#[derive(Debug, Clone)]
pub struct IoLoopConfig {
    /// Buffer size for each port
    pub buffer_size: usize,
    /// Read timeout in milliseconds
    pub read_timeout_ms: u64,
    /// Event channel capacity
    pub event_channel_size: usize,
}

impl Default for IoLoopConfig {
    fn default() -> Self {
        Self {
            buffer_size: 4096,
            read_timeout_ms: 100,
            event_channel_size: 100,
        }
    }
}

/// Async I/O loop
pub struct IoLoop {
    config: IoLoopConfig,
    port_manager: PortManager,
    event_tx: mpsc::Sender<IoEvent>,
    event_rx: Option<mpsc::Receiver<IoEvent>>,
    active_ports: Arc<Mutex<HashMap<String, bool>>>,
}

impl IoLoop {
    /// Create a new I/O loop
    pub fn new() -> Self {
        let config = IoLoopConfig::default();
        Self::with_config(config)
    }

    /// Create a new I/O loop with custom configuration
    pub fn with_config(config: IoLoopConfig) -> Self {
        let (event_tx, event_rx) = mpsc::channel(config.event_channel_size);

        Self {
            config,
            port_manager: PortManager::new(),
            event_tx,
            event_rx: Some(event_rx),
            active_ports: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get a channel sender to subscribe to events
    pub fn event_sender(&self) -> mpsc::Sender<IoEvent> {
        self.event_tx.clone()
    }

    /// Add a port to the I/O loop
    pub async fn add_port(&self, port_name: &str) -> Result<String> {
        use crate::serial_core::SerialConfig;

        let config = SerialConfig::default();
        let port_id = self.port_manager.open_port(port_name, config).await?;

        // Mark as active
        let mut ports = self.active_ports.lock().await;
        ports.insert(port_id.clone(), true);

        // Send event
        let _ = self.event_tx.send(IoEvent::PortOpened {
            port_id: port_id.clone(),
        }).await;

        Ok(port_id)
    }

    /// Remove a port from the I/O loop
    pub async fn remove_port(&self, port_id: &str) -> Result<()> {
        self.port_manager.close_port(port_id).await?;

        // Mark as inactive
        let mut ports = self.active_ports.lock().await;
        ports.remove(port_id);

        // Send event
        let _ = self.event_tx.send(IoEvent::PortClosed {
            port_id: port_id.to_string(),
        }).await;

        Ok(())
    }

    /// Run the I/O loop
    pub async fn run(&mut self) -> Result<()> {
        let mut event_rx = self.event_rx.take()
            .ok_or_else(|| SerialError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Event receiver already taken"
            )))?;

        // Spawn I/O tasks for each active port
        let active_ports = self.active_ports.clone();
        let port_manager = self.port_manager.clone();
        let event_tx = self.event_tx.clone();
        let config = self.config.clone();

        // I/O task
        let io_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(config.read_timeout_ms));

            loop {
                interval.tick().await;

                // Get list of active ports
                let ports = {
                    let ports_guard = active_ports.lock().await;
                    ports_guard.keys().cloned().collect::<Vec<_>>()
                };

                // Try to read from each port
                for port_id in ports {
                    let port_handle = match port_manager.get_port(&port_id).await {
                        Ok(handle) => handle,
                        Err(_) => continue,
                    };

                    let mut handle = port_handle.lock().await;
                    let mut buffer = vec![0u8; config.buffer_size];

                    // Non-blocking read with timeout
                    match timeout(Duration::from_millis(10), async {
                        handle.read(&mut buffer)
                    }).await {
                        Ok(Ok(n)) if n > 0 => {
                            buffer.truncate(n);

                            let _ = event_tx.send(IoEvent::DataReceived {
                                port_id: port_id.clone(),
                                data: buffer,
                            }).await;
                        }
                        Ok(Ok(_)) => {
                            // No data available
                        }
                        Ok(Err(e)) => {
                            let _ = event_tx.send(IoEvent::Error {
                                port_id: port_id.clone(),
                                error: format!("{:?}", e),
                            }).await;
                        }
                        Err(_) => {
                            // Timeout - no data available
                        }
                    }
                }
            }
        });

        // Event processing loop
        while let Some(event) = event_rx.recv().await {
            match event {
                IoEvent::DataReceived { port_id, data } => {
                    tracing::debug!("Received {} bytes from {}", data.len(), port_id);
                }
                IoEvent::DataSent { port_id, length } => {
                    tracing::debug!("Sent {} bytes to {}", length, port_id);
                }
                IoEvent::PortOpened { port_id } => {
                    tracing::info!("Port opened: {}", port_id);
                }
                IoEvent::PortClosed { port_id } => {
                    tracing::info!("Port closed: {}", port_id);
                }
                IoEvent::Error { port_id, error } => {
                    tracing::error!("Error on port {}: {}", port_id, error);
                }
            }
        }

        // Clean up I/O task
        io_task.abort();

        Ok(())
    }

    /// Write data to a port
    pub async fn write(&self, port_id: &str, data: &[u8]) -> Result<()> {
        let port_handle = self.port_manager.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;

        let bytes_written = handle.write(data)?;

        // Send event
        let _ = self.event_tx.send(IoEvent::DataSent {
            port_id: port_id.to_string(),
            length: bytes_written,
        }).await;

        Ok(())
    }

    /// Read data from a port (blocking)
    pub async fn read(&self, port_id: &str, buf: &mut [u8]) -> Result<usize> {
        let port_handle = self.port_manager.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;

        let bytes_read = handle.read(buf)?;

        Ok(bytes_read)
    }
}

impl Default for IoLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_loop_creation() {
        let io_loop = IoLoop::new();
        assert!(io_loop.event_rx.is_some());
    }

    #[test]
    fn test_io_loop_config_default() {
        let config = IoLoopConfig::default();
        assert_eq!(config.buffer_size, 4096);
        assert_eq!(config.read_timeout_ms, 100);
        assert_eq!(config.event_channel_size, 100);
    }

    #[tokio::test]
    async fn test_event_channel() {
        let mut io_loop = IoLoop::new();
        let mut rx = io_loop.event_rx.take().unwrap();

        // Send test event
        let _ = io_loop.event_tx.send(IoEvent::PortOpened {
            port_id: "test".to_string(),
        }).await;

        // Receive event
        let event = rx.recv().await.unwrap();
        match event {
            IoEvent::PortOpened { port_id } => {
                assert_eq!(port_id, "test");
            }
            _ => panic!("Unexpected event"),
        }
    }
}
