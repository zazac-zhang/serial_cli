//! Serial port management
//!
//! This module provides serial port discovery, configuration, and management.

use crate::error::{Result, SerialError, SerialPortError};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_serial::SerialPort;

/// Serial port manager
#[derive(Clone)]
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
    pub flow_control: FlowControl,
    pub dtr_enable: bool,
    pub rts_enable: bool,
}

/// Flow control setting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowControl {
    None,
    Software, // XON/XOFF
    Hardware, // RTS/CTS
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
            flow_control: FlowControl::None,
            dtr_enable: true,
            rts_enable: true,
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
                ports
                    .into_iter()
                    .map(|p| {
                        let info = SerialPortInfo {
                            port_name: p.port_name.clone(),
                            port_type: format!("{:?}", p.port_type),
                            friendly_name: None,
                            hardware_id: None,
                            manufacturer: None,
                            com_number: None,
                        };

                        // Try to extract COM port number on Windows
                        #[cfg(target_os = "windows")]
                        {
                            if let Some(com_str) = p.port_name.strip_prefix("COM") {
                                if let Ok(num) = com_str.parse::<u32>() {
                                    info.com_number = Some(num);
                                }
                            }

                            // Try to get friendly name from port type info
                            // Note: This is a placeholder - real implementation would
                            // query Windows registry or device manager
                            if info.com_number.is_some() {
                                info.friendly_name = Some(format!("Serial Port {}", p.port_name));
                            }
                        }

                        info
                    })
                    .collect()
            })
    }

    /// Open a serial port
    pub async fn open_port(&self, name: &str, config: SerialConfig) -> Result<String> {
        // Check if port is already open
        let ports_guard = self.ports.lock().await;
        if ports_guard.contains_key(name) {
            return Err(SerialError::Serial(SerialPortError::port_busy(
                name,
                Some("Port is already opened by this application"),
            )));
        }
        drop(ports_guard);

        // Build serial port
        let mut builder = tokio_serial::new(name, config.baudrate);

        builder = builder.timeout(Duration::from_millis(config.timeout_ms));

        // Configure data bits
        let data_bits = match config.databits {
            5 => tokio_serial::DataBits::Five,
            6 => tokio_serial::DataBits::Six,
            7 => tokio_serial::DataBits::Seven,
            8 => tokio_serial::DataBits::Eight,
            _ => tokio_serial::DataBits::Eight,
        };
        builder = builder.data_bits(data_bits);

        // Configure parity
        let parity = match config.parity {
            Parity::None => tokio_serial::Parity::None,
            Parity::Odd => tokio_serial::Parity::Odd,
            Parity::Even => tokio_serial::Parity::Even,
        };
        builder = builder.parity(parity);

        // Configure stop bits
        let stop_bits = match config.stopbits {
            1 => tokio_serial::StopBits::One,
            2 => tokio_serial::StopBits::Two,
            _ => tokio_serial::StopBits::One,
        };
        builder = builder.stop_bits(stop_bits);

        // Configure flow control
        let flow_control = match config.flow_control {
            FlowControl::None => tokio_serial::FlowControl::None,
            FlowControl::Software => tokio_serial::FlowControl::Software,
            FlowControl::Hardware => tokio_serial::FlowControl::Hardware,
        };
        builder = builder.flow_control(flow_control);

        // Open the port
        let port = builder.open().map_err(|e| {
            // Map tokio-serial errors to our error types
            let error_msg = e.to_string();
            if error_msg.contains("permission denied")
                || error_msg.contains("Permission denied")
                || error_msg.contains("Access is denied")
            {
                SerialError::Serial(SerialPortError::permission_denied(
                    name,
                    Some("Try running as Administrator or check port permissions"),
                ))
            } else if error_msg.contains("not found")
                || error_msg.contains("No such file")
                || error_msg.contains("The system cannot find the file")
            {
                SerialError::Serial(SerialPortError::PortNotFound(name.to_string()))
            } else if error_msg.contains("busy")
                || error_msg.contains("Busy")
                || error_msg.contains("used by another application")
            {
                SerialError::Serial(SerialPortError::port_busy(
                    name,
                    Some("Close other applications using this port or try a different port"),
                ))
            } else {
                SerialError::Serial(SerialPortError::IoError(error_msg))
            }
        })?;

        // Note: DTR/RTS control would require platform-specific implementation
        // For now, we'll just log if non-default settings are requested
        if !config.dtr_enable || !config.rts_enable {
            tracing::warn!("DTR/RTS control requested but not yet implemented");
        }

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
        ports_guard.remove(port_id).ok_or_else(|| {
            SerialError::Serial(SerialPortError::PortNotFound(port_id.to_string()))
        })?;
        Ok(())
    }

    /// Get a port handle by ID
    pub async fn get_port(&self, port_id: &str) -> Result<Arc<Mutex<SerialPortHandle>>> {
        let ports_guard = self.ports.lock().await;
        ports_guard
            .get(port_id)
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
        self.port
            .write(data)
            .map_err(|e| SerialError::Serial(SerialPortError::IoError(e.to_string())))
    }

    /// Read data from the port
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.port
            .read(buf)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub com_number: Option<u32>,
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
        let result = manager
            .open_port("NONEXISTENT", SerialConfig::default())
            .await;
        assert!(result.is_err());
    }
}
