//! Serial port management
//!
//! This module provides serial port discovery, configuration, and management.

use crate::error::{Result, SerialError, SerialPortError};
use crate::serial_core::signals::PlatformSignals;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// Serial port manager
#[derive(Clone)]
pub struct PortManager {
    ports: Arc<Mutex<HashMap<String, Arc<Mutex<SerialPortHandle>>>>>,
    io_loop_enabled: Arc<Mutex<bool>>,
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
}

/// Serial port handle
pub struct SerialPortHandle {
    name: String,
    port: Box<dyn serialport::SerialPort>,
    config: SerialConfig,
    protocol: Option<String>,
    dtr_state: bool,
    rts_state: bool,
    #[cfg(unix)]
    signal_controller: crate::serial_core::signals::UnixSignalController,
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
    Software,
    Hardware,
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
    /// List available serial ports
    pub fn list_ports(&self) -> Result<Vec<SerialPortInfo>> {
        tokio_serial::available_ports()
            .map_err(|e| SerialError::Serial(SerialPortError::IoError(e.to_string())))
            .map(|ports| {
                ports
                    .into_iter()
                    .map(|p| {
                        #[cfg_attr(not(target_os = "windows"), allow(unused_mut))]
                        let mut info = SerialPortInfo {
                            port_name: p.port_name.clone(),
                            port_type: format!("{:?}", p.port_type),
                            friendly_name: None,
                            hardware_id: None,
                            manufacturer: None,
                            com_number: None,
                        };

                        #[cfg(target_os = "windows")]
                        {
                            if let Some(com_str) = p.port_name.strip_prefix("COM") {
                                if let Ok(num) = com_str.parse::<u32>() {
                                    info.com_number = Some(num);
                                }
                            }
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
        let ports_guard = self.ports.lock().await;
        if ports_guard.contains_key(name) {
            return Err(SerialError::Serial(SerialPortError::port_busy(
                name,
                Some("Port is already opened by this application"),
            )));
        }
        drop(ports_guard);

        // Open via serialport::TTYPort on Unix to get raw fd for DTR/RTS
        #[cfg(unix)]
        let (port, signal_controller) = {
            use std::os::unix::io::AsRawFd;

            let builder = serialport::new(name, config.baudrate)
                .timeout(Duration::from_millis(config.timeout_ms))
                .data_bits(match config.databits {
                    5 => serialport::DataBits::Five,
                    6 => serialport::DataBits::Six,
                    7 => serialport::DataBits::Seven,
                    8 => serialport::DataBits::Eight,
                    _ => serialport::DataBits::Eight,
                })
                .parity(match config.parity {
                    Parity::None => serialport::Parity::None,
                    Parity::Odd => serialport::Parity::Odd,
                    Parity::Even => serialport::Parity::Even,
                })
                .stop_bits(match config.stopbits {
                    1 => serialport::StopBits::One,
                    2 => serialport::StopBits::Two,
                    _ => serialport::StopBits::One,
                })
                .flow_control(match config.flow_control {
                    FlowControl::None => serialport::FlowControl::None,
                    FlowControl::Software => serialport::FlowControl::Software,
                    FlowControl::Hardware => serialport::FlowControl::Hardware,
                });

            // Open as TTYPort to get raw fd
            let tty = builder.open_native().map_err(|e| map_serial_error(e, name))?;
            let fd = tty.as_raw_fd();

            // Set DTR/RTS via real ioctl
            set_dtr_on_fd(fd, config.dtr_enable);
            set_rts_on_fd(fd, config.rts_enable);

            // Create signal controller with correct initial state
            let mut signal_controller = crate::serial_core::signals::UnixSignalController::new();
            signal_controller.set_dtr(config.dtr_enable).ok();
            signal_controller.set_rts(config.rts_enable).ok();

            // Keep TTYPort directly — it implements serialport::SerialPort with blocking I/O
            let port: Box<dyn serialport::SerialPort> = Box::new(tty);

            (port, signal_controller)
        };

        #[cfg(not(unix))]
        let (port, _signal_controller) = {
            // On Windows, use serialport::new().open_native() to get COMPort
            let builder = serialport::new(name, config.baudrate)
                .timeout(Duration::from_millis(config.timeout_ms))
                .data_bits(match config.databits {
                    5 => serialport::DataBits::Five,
                    6 => serialport::DataBits::Six,
                    7 => serialport::DataBits::Seven,
                    8 => serialport::DataBits::Eight,
                    _ => serialport::DataBits::Eight,
                })
                .parity(match config.parity {
                    Parity::None => serialport::Parity::None,
                    Parity::Odd => serialport::Parity::Odd,
                    Parity::Even => serialport::Parity::Even,
                })
                .stop_bits(match config.stopbits {
                    1 => serialport::StopBits::One,
                    2 => serialport::StopBits::Two,
                    _ => serialport::StopBits::One,
                })
                .flow_control(match config.flow_control {
                    FlowControl::None => serialport::FlowControl::None,
                    FlowControl::Software => serialport::FlowControl::Software,
                    FlowControl::Hardware => serialport::FlowControl::Hardware,
                });

            let port = builder.open_native().map_err(|e| map_serial_error(e, name))?;
            (Box::new(port), ())
        };

        let dtr_state = config.dtr_enable;
        let rts_state = config.rts_enable;
        let handle = SerialPortHandle {
            name: name.to_string(),
            port,
            config,
            protocol: None,
            dtr_state,
            rts_state,
            #[cfg(unix)]
            signal_controller,
        };

        let mut ports_guard = self.ports.lock().await;
        let port_id = format!("{}-{}", name, uuid::Uuid::new_v4());
        let port_handle = Arc::new(Mutex::new(handle));
        ports_guard.insert(port_id.clone(), port_handle.clone());

        if *self.io_loop_enabled.lock().await {
            let port_id_clone = port_id.clone();
            let port_handle_clone = port_handle.clone();

            tokio::spawn(async move {
                let mut buffer = vec![0u8; 4096];
                loop {
                    let mut handle = port_handle_clone.lock().await;
                    match handle.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            let data = buffer[..n].to_vec();
                            tracing::debug!("IoLoop: Received {} bytes from {}", n, port_id_clone);
                            if let Ok(text) = String::from_utf8(data.clone()) {
                                tracing::debug!("Data: {}", text);
                            }
                        }
                        Ok(_) => {}
                        Err(_) => break,
                    }
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
    pub async fn set_port_protocol(
        &self,
        port_id: &str,
        protocol_name: Option<String>,
    ) -> Result<()> {
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

#[cfg(unix)]
fn set_dtr_on_fd(fd: libc::c_int, enable: bool) {
    let result = unsafe {
        crate::serial_core::signals::UnixSignalController::set_modem_bit_on_fd(
            fd,
            libc::TIOCM_DTR,
            enable,
        )
    };
    if let Err(e) = result {
        tracing::warn!("Failed to set DTR on open: {}", e);
    }
}

#[cfg(unix)]
fn set_rts_on_fd(fd: libc::c_int, enable: bool) {
    let result = unsafe {
        crate::serial_core::signals::UnixSignalController::set_modem_bit_on_fd(
            fd,
            libc::TIOCM_RTS,
            enable,
        )
    };
    if let Err(e) = result {
        tracing::warn!("Failed to set RTS on open: {}", e);
    }
}

fn map_serial_error(e: serialport::Error, name: &str) -> SerialError {
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

    /// Set DTR signal state
    pub fn set_dtr(&mut self, enable: bool) -> Result<()> {
        if enable != self.dtr_state {
            let old_state = self.dtr_state;
            self.dtr_state = enable;

            #[cfg(unix)]
            let result = self.signal_controller.set_dtr(enable);

            #[cfg(not(unix))]
            let result: Result<crate::serial_core::signals::SignalState> = {
                self.dtr_state = enable;
                Ok(crate::serial_core::signals::SignalState::NotSupported)
            };

            match result {
                Ok(state) if matches!(state, crate::serial_core::signals::SignalState::Set(_)) => {
                    tracing::info!("DTR signal set to: {} for port {}", enable, self.name);
                }
                Ok(_) => {
                    tracing::warn!(
                        "DTR signal control not available on this platform for port {}",
                        self.name
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to set DTR signal for port {}: {}. State updated in memory only.",
                        self.name, e
                    );
                    self.dtr_state = old_state;
                }
            }
        }
        Ok(())
    }

    /// Set RTS signal state
    pub fn set_rts(&mut self, enable: bool) -> Result<()> {
        if enable != self.rts_state {
            let old_state = self.rts_state;
            self.rts_state = enable;

            #[cfg(unix)]
            let result = self.signal_controller.set_rts(enable);

            #[cfg(not(unix))]
            let result: Result<crate::serial_core::signals::SignalState> = {
                self.rts_state = enable;
                Ok(crate::serial_core::signals::SignalState::NotSupported)
            };

            match result {
                Ok(state) if matches!(state, crate::serial_core::signals::SignalState::Set(_)) => {
                    tracing::info!("RTS signal set to: {} for port {}", enable, self.name);
                }
                Ok(_) => {
                    tracing::warn!(
                        "RTS signal control not available on this platform for port {}",
                        self.name
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to set RTS signal for port {}: {}. State updated in memory only.",
                        self.name, e
                    );
                    self.rts_state = old_state;
                }
            }
        }
        Ok(())
    }

    /// Get current DTR state
    pub fn dtr_enabled(&self) -> bool {
        self.dtr_state
    }

    /// Get current RTS state
    pub fn rts_enabled(&self) -> bool {
        self.rts_state
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
    }

    #[tokio::test]
    async fn test_port_manager_creation() {
        let manager = PortManager::new();
        let result = manager
            .open_port("NONEXISTENT", SerialConfig::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_nonexistent_port() {
        let manager = PortManager::new();
        let result = manager.close_port("nonexistent_id").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_config_custom_baudrate() {
        let mut config = SerialConfig::default();
        config.baudrate = 9600;
        assert_eq!(config.baudrate, 9600);
    }

    #[test]
    fn test_config_all_fields() {
        let mut config = SerialConfig::default();
        config.baudrate = 57600;
        config.databits = 7;
        config.parity = Parity::Even;
        assert_eq!(config.baudrate, 57600);
        assert_eq!(config.databits, 7);
        assert!(matches!(config.parity, Parity::Even));
    }
}
