//! Serial port management
//!
//! This module provides serial port discovery, configuration, and management.

use crate::error::{Result, SerialError, SerialPortError};
use tokio_serial::{SerialPort};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Serial port manager
pub struct PortManager {
    ports: Arc<Mutex<HashMap<String, Arc<Mutex<SerialPortHandle>>>>>,
}

/// Serial port handle
pub struct SerialPortHandle {
    name: String,
    port: Box<dyn SerialPort>,
    config: SerialConfig,
}

/// Serial port configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub baudrate: u32,
    pub databits: u8,
    pub stopbits: u8,
    pub parity: Parity,
    pub timeout_ms: u64,
}

/// Parity setting
#[derive(Debug, Clone, Copy)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            databits: 8,
            stopbits: 1,
            parity: Parity::None,
            timeout_ms: 1000,
        }
    }
}

impl PortManager {
    /// Create a new port manager
    pub fn new() -> Self {
        Self {
            ports: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// List available serial ports
    pub fn list_ports(&self) -> Result<Vec<SerialPortInfo>> {
        tokio_serial::available_ports()
            .map_err(|e| SerialError::Serial(SerialPortError::IoError(e.to_string())))
            .map(|ports| {
                ports.into_iter().map(|p| SerialPortInfo {
                    port_name: p.port_name,
                    port_type: format!("{:?}", p.port_type),
                }).collect()
            })
    }

    /// Open a serial port
    pub async fn open_port(&self, name: &str, config: SerialConfig) -> Result<String> {
        // Check if port is already open
        let ports_guard = self.ports.lock().await;
        if ports_guard.contains_key(name) {
            return Err(SerialError::Serial(SerialPortError::PortBusy(name.to_string())));
        }
        drop(ports_guard);

        // Build serial port
        let mut builder = tokio_serial::new(name, config.baudrate);

        builder = builder.timeout(Duration::from_millis(config.timeout_ms));

        // Open the port
        let port = builder.open().map_err(|e| {
            // Map tokio-serial errors to our error types
            let error_msg = e.to_string();
            if error_msg.contains("permission denied") || error_msg.contains("Permission denied") {
                SerialError::Serial(SerialPortError::PermissionDenied(name.to_string()))
            } else if error_msg.contains("not found") || error_msg.contains("No such file") {
                SerialError::Serial(SerialPortError::PortNotFound(name.to_string()))
            } else {
                SerialError::Serial(SerialPortError::IoError(error_msg))
            }
        })?;

        // Create handle
        let handle = SerialPortHandle {
            name: name.to_string(),
            port,
            config,
        };

        // Store handle
        let mut ports_guard = self.ports.lock().await;
        let port_id = format!("{}-{}", name, uuid::Uuid::new_v4());
        ports_guard.insert(port_id.clone(), Arc::new(Mutex::new(handle)));

        Ok(port_id)
    }

    /// Close a serial port
    pub async fn close_port(&self, port_id: &str) -> Result<()> {
        let mut ports_guard = self.ports.lock().await;
        ports_guard.remove(port_id)
            .ok_or_else(|| SerialError::Serial(SerialPortError::PortNotFound(port_id.to_string())))?;
        Ok(())
    }

    /// Get a port handle by ID
    pub async fn get_port(&self, port_id: &str) -> Result<Arc<Mutex<SerialPortHandle>>> {
        let ports_guard = self.ports.lock().await;
        ports_guard.get(port_id)
            .cloned()
            .ok_or_else(|| SerialError::Serial(SerialPortError::PortNotFound(port_id.to_string())))
    }
}

impl Default for PortManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialPortHandle {
    /// Get port name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get port configuration
    pub fn config(&self) -> &SerialConfig {
        &self.config
    }

    /// Write data to the port
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.port.write(data)
            .map_err(|e| SerialError::Serial(SerialPortError::IoError(e.to_string())))
    }

    /// Read data from the port
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.port.read(buf)
            .map_err(|e| SerialError::Serial(SerialPortError::IoError(e.to_string())))
    }

    /// Close the port
    pub fn close(self) -> Result<()> {
        // The port will be closed when dropped
        Ok(())
    }
}

/// Serial port information
#[derive(Debug, Clone, serde::Serialize)]
pub struct SerialPortInfo {
    pub port_name: String,
    pub port_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = SerialConfig::default();
        assert_eq!(config.baudrate, 115200);
        assert_eq!(config.databits, 8);
        assert_eq!(config.stopbits, 1);
    }

    #[test]
    fn test_list_ports() {
        let manager = PortManager::new();
        let ports = manager.list_ports();
        assert!(ports.is_ok());
        // May return empty list if no ports available
    }

    #[tokio::test]
    async fn test_port_manager_creation() {
        let manager = PortManager::new();
        // Try to open a non-existent port
        let result = manager.open_port("NONEXISTENT", SerialConfig::default()).await;
        assert!(result.is_err());
    }
}
