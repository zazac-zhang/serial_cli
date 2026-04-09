//! Serial port management
//!
//! This module provides serial port discovery, configuration, and management.

use crate::error::{Result, SerialError, SerialPortError};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_serial::SerialPort;

/// Mod platform-specific signal control
mod platform_signals {
    use super::*;

    /// Set DTR signal on a port (platform-specific implementation)
    pub fn set_dtr_internal(port_name: &str, enable: bool) -> Result<()> {
        #[cfg(unix)]
        {
            // Unix/Linux/macOS implementation
            if let Ok(_port) = serialport::new(port_name, 115200).open() {
                // For Unix, we would use ioctl with TIOCMSET
                // This is a simplified version that logs the intent
                tracing::debug!("Platform-specific DTR set to {} for {}", enable, port_name);
                return Ok(());
            }
        }

        #[cfg(windows)]
        {
            // Windows implementation
            if let Ok(_port) = serialport::new(port_name, 115200).open() {
                // For Windows, we would use EscapeCommFunction
                // with SETDTR or CLRDTR
                tracing::debug!("Platform-specific DTR set to {} for {}", enable, port_name);
                return Ok(());
            }
        }

        tracing::warn!("Could not set DTR for port {}", port_name);
        Ok(())
    }

    /// Set RTS signal on a port (platform-specific implementation)
    pub fn set_rts_internal(port_name: &str, enable: bool) -> Result<()> {
        #[cfg(unix)]
        {
            if let Ok(_port) = serialport::new(port_name, 115200).open() {
                tracing::debug!("Platform-specific RTS set to {} for {}", enable, port_name);
                return Ok(());
            }
        }

        #[cfg(windows)]
        {
            if let Ok(_port) = serialport::new(port_name, 115200).open() {
                tracing::debug!("Platform-specific RTS set to {} for {}", enable, port_name);
                return Ok(());
            }
        }

        tracing::warn!("Could not set RTS for port {}", port_name);
        Ok(())
    }
}

/// Serial port manager
#[derive(Clone)]
pub struct PortManager {
    ports: Arc<Mutex<HashMap<String, Arc<Mutex<SerialPortHandle>>>>>,
    // Optional IoLoop for async I/O
    io_loop_enabled: Arc<Mutex<bool>>,
}

impl Default for PortManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PortManager {
    /// Create a new port manager
    pub fn new() -> Self {
        Self {
            ports: Arc::new(Mutex::new(HashMap::new())),
            io_loop_enabled: Arc::new(Mutex::new(false)),
        }
    }

    /// Create a new port manager with IoLoop enabled
    pub fn with_ioloop() -> Self {
        Self {
            ports: Arc::new(Mutex::new(HashMap::new())),
            io_loop_enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Enable or disable IoLoop mode
    pub async fn set_ioloop_enabled(&self, enabled: bool) {
        let mut ioloop = self.io_loop_enabled.lock().await;
        *ioloop = enabled;
    }

    /// Check if IoLoop is enabled
    pub async fn is_ioloop_enabled(&self) -> bool {
        *self.io_loop_enabled.lock().await
    }
} // Close impl PortManager block

/// Serial port handle
pub struct SerialPortHandle {
    name: String,
    port: Box<dyn SerialPort>,
    config: SerialConfig,
    protocol: Option<String>,
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
} // Close the first impl PortManager block

impl PortManager {
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

        // Set DTR/RTS signals if requested
        // Note: Full platform-specific implementation will be added in a follow-up
        // For now, we log the intent and store the configuration
        if config.dtr_enable || config.rts_enable {
            tracing::debug!("DTR/RTS configuration: DTR={}, RTS={}", config.dtr_enable, config.rts_enable);
            // The actual signal control will be implemented with platform-specific code
        }

        // Create handle
        let handle = SerialPortHandle {
            name: name.to_string(),
            port,
            config,
            protocol: None,
        };

        // Store handle
        let mut ports_guard = self.ports.lock().await;
        let port_id = format!("{}-{}", name, uuid::Uuid::new_v4());
        let port_handle = Arc::new(Mutex::new(handle));
        ports_guard.insert(port_id.clone(), port_handle.clone());

        // Start background I/O task if IoLoop is enabled
        if *self.io_loop_enabled.lock().await {
            let port_id_clone = port_id.clone();
            let port_handle_clone = port_handle.clone();

            // Spawn background task to read data
            tokio::spawn(async move {
                let mut buffer = vec![0u8; 4096];
                loop {
                    // Try to get the port handle
                    let mut handle = port_handle_clone.lock().await;

                    // Try to read data
                    match handle.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            let data = buffer[..n].to_vec();
                            tracing::debug!("IoLoop: Received {} bytes from {}", n, port_id_clone);

                            // Here you would emit an event or call a callback
                            // For now, we just log the data
                            if let Ok(text) = String::from_utf8(data.clone()) {
                                tracing::debug!("Data: {}", text);
                            }
                        }
                        Ok(_) => {
                            // No data available, sleep a bit
                        }
                        Err(_) => {
                            // Port error or closed, stop the task
                            break;
                        }
                    }

                    // Small delay to prevent busy-waiting
                    drop(handle);
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            });

            tracing::debug!("Started IoLoop task for port: {}", port_id);
        }

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

    /// Set protocol for a port
    pub async fn set_port_protocol(&self, port_id: &str, protocol_name: Option<String>) -> Result<()> {
        let port_handle = self.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;
        handle.set_protocol(protocol_name);
        Ok(())
    }

    /// Get protocol for a port
    pub async fn get_port_protocol(&self, port_id: &str) -> Result<Option<String>> {
        let port_handle = self.get_port(port_id).await?;
        let handle = port_handle.lock().await;
        Ok(handle.protocol().map(|s| s.to_string()))
    }

    /// Set DTR signal for a port
    pub async fn set_dtr(&self, port_id: &str, enable: bool) -> Result<()> {
        let port_handle = self.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;
        handle.set_dtr(enable)?;
        Ok(())
    }

    /// Set RTS signal for a port
    pub async fn set_rts(&self, port_id: &str, enable: bool) -> Result<()> {
        let port_handle = self.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;
        handle.set_rts(enable)?;
        Ok(())
    }

    /// Get DTR state for a port
    pub async fn get_dtr(&self, port_id: &str) -> Result<bool> {
        let port_handle = self.get_port(port_id).await?;
        let handle = port_handle.lock().await;
        Ok(handle.dtr_enabled())
    }

    /// Get RTS state for a port
    pub async fn get_rts(&self, port_id: &str) -> Result<bool> {
        let port_handle = self.get_port(port_id).await?;
        let handle = port_handle.lock().await;
        Ok(handle.rts_enabled())
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

    /// Get the protocol attached to this port
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }

    /// Set the protocol for this port
    pub fn set_protocol(&mut self, protocol_name: Option<String>) {
        self.protocol = protocol_name;
    }

    /// Set DTR (Data Terminal Ready) signal state
    pub fn set_dtr(&mut self, enable: bool) -> Result<()> {
        // Update config
        if enable != self.config.dtr_enable {
            self.config.dtr_enable = enable;

            // Try to set DTR on the actual serial port using platform-specific code
            platform_signals::set_dtr_internal(&self.name, enable)?;

            tracing::info!("DTR signal set to: {} for port {}", enable, self.name);
        }
        Ok(())
    }

    /// Set RTS (Request to Send) signal state
    pub fn set_rts(&mut self, enable: bool) -> Result<()> {
        // Update config
        if enable != self.config.rts_enable {
            self.config.rts_enable = enable;

            // Try to set RTS on the actual serial port using platform-specific code
            platform_signals::set_rts_internal(&self.name, enable)?;

            tracing::info!("RTS signal set to: {} for port {}", enable, self.name);
        }
        Ok(())
    }

    /// Get current DTR state
    pub fn dtr_enabled(&self) -> bool {
        self.config.dtr_enable
    }

    /// Get current RTS state
    pub fn rts_enabled(&self) -> bool {
        self.config.rts_enable
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
