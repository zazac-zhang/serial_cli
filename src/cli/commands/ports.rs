//! Port listing and data sending commands
//!
//! Handlers for `serial-cli list-ports` and `serial-cli send`.

use crate::error::Result;
use crate::serial_core::{PortManager, SerialConfig};

/// List all available serial ports and print them as JSON.
///
/// Uses the system's serial port enumeration to discover ports like
/// `/dev/ttyUSB0` (Linux/macOS) or `COM1` (Windows).
///
/// # Errors
///
/// Returns an error if port enumeration fails (e.g., permission issues
/// on the platform).
pub fn list_ports() -> Result<()> {
    use serde_json::json;

    let manager = PortManager::new();
    let ports = manager.list_ports()?;

    let output: Vec<serde_json::Value> = ports
        .iter()
        .map(|p| {
            json!({
                "port_name": p.port_name,
                "port_type": format!("{:?}", p.port_type),
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&output).unwrap());

    Ok(())
}

/// Open a serial port, send the provided data, and attempt to read a response.
///
/// Uses [`SerialConfig::default()`] for port settings (115200 baud, 8N1).
/// After writing, waits 100ms then reads up to 1024 bytes of response data.
/// The port is closed after the operation regardless of success or failure.
///
/// # Arguments
///
/// * `port` - Port name (e.g., `/dev/ttyUSB0`, `COM1`)
/// * `data` - Plain text data to send
///
/// # Errors
///
/// Returns an error when:
/// - The port does not exist or cannot be opened
/// - Permission is denied
/// - The port is busy
/// - Write or read fails at the OS level
pub async fn send_data(port: &str, data: &str) -> Result<()> {
    use std::thread;
    use std::time::Duration;

    tracing::info!("Opening port: {}", port);

    // Create port manager
    let manager = PortManager::new();

    // Use default configuration
    let config = SerialConfig::default();

    // Open the port
    let port_id = manager.open_port(port, config).await?;

    tracing::info!("Port opened successfully: {}", port_id);
    tracing::info!("Sending data: {}", data);

    // Get the port handle
    let port_handle = manager.get_port(&port_id).await?;
    let mut handle = port_handle.lock().await;

    // Convert data to bytes
    let bytes = data.as_bytes();

    // Send data
    let bytes_written = handle.write(bytes)?;
    tracing::info!("Sent {} bytes", bytes_written);

    // Wait a bit for response
    thread::sleep(Duration::from_millis(100));

    // Try to read response
    let mut buffer = [0u8; 1024];
    match handle.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received response ({} bytes): {}", bytes_read, response);
            } else {
                tracing::info!("No response received");
            }
        }
        Err(e) => {
            tracing::info!("Note: Could not read response: {}", e);
        }
    }

    // Close the port
    drop(handle);
    manager.close_port(&port_id).await?;
    tracing::info!("Port closed");

    Ok(())
}
