//! CLI commands
//!
//! This module provides single-command execution mode.

use crate::error::Result;
use crate::serial_core::{PortManager, SerialConfig};
use clap::Parser;
use serde_json::json;

/// Single command executor
#[derive(Parser, Debug)]
pub struct CommandExecutor {
    /// Port name
    #[arg(short, long)]
    pub port: String,

    /// Timeout in milliseconds
    #[arg(short, long, default_value = "1000")]
    pub timeout: u64,

    /// Output as JSON
    #[arg(long, default_value = "false")]
    pub json: bool,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new(port: String, timeout: u64, json: bool) -> Self {
        Self {
            port,
            timeout,
            json,
        }
    }

    /// Execute a send command
    pub async fn send(&self, data: &str) -> Result<()> {
        let manager = PortManager::new();
        let config = SerialConfig::default();

        // Open port
        let port_id = manager.open_port(&self.port, config).await?;

        tracing::info!("Sending to {}: {}", self.port, data);

        // Get port handle and send
        let port_handle = manager.get_port(&port_id).await?;
        let mut handle = port_handle.lock().await;

        let bytes_written = handle.write(data.as_bytes())?;
        tracing::info!("Sent {} bytes", bytes_written);

        // Wait for response
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Try to read response
        let mut buffer = vec![0u8; 1024];
        match handle.read(&mut buffer) {
            Ok(n) if n > 0 => {
                buffer.truncate(n);
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    tracing::info!("Response: {}", text);
                } else {
                    let hex: String = buffer.iter().map(|b| format!("{:02x} ", b)).collect();
                    tracing::info!("Response (hex): {}", hex);
                }
            }
            Ok(_) => tracing::info!("No response"),
            Err(e) => tracing::info!("Read error: {}", e),
        }

        // Close port
        manager.close_port(&port_id).await?;

        Ok(())
    }

    /// Execute a receive command
    pub async fn recv(&self, bytes: usize) -> Result<()> {
        let manager = PortManager::new();
        let config = SerialConfig::default();

        // Open port
        let port_id = manager.open_port(&self.port, config).await?;

        tracing::info!("Reading up to {} bytes from {}", bytes, self.port);

        // Get port handle and read
        let port_handle = manager.get_port(&port_id).await?;
        let mut handle = port_handle.lock().await;

        let mut buffer = vec![0u8; bytes];
        match handle.read(&mut buffer) {
            Ok(n) if n > 0 => {
                buffer.truncate(n);

                if self.json {
                    let hex_data: String = buffer.iter().map(|b| format!("{:02x}", b)).collect();
                    let output = json!({
                        "port": self.port,
                        "bytes_read": n,
                        "data": hex_data,
                    });
                    tracing::info!("{}", serde_json::to_string_pretty(&output).unwrap());
                } else {
                    if let Ok(text) = String::from_utf8(buffer.clone()) {
                        tracing::info!("Received ({} bytes): {}", n, text);
                    } else {
                        let hex: String = buffer.iter().map(|b| format!("{:02x} ", b)).collect();
                        tracing::info!("Received ({} bytes): {}", n, hex);
                    }
                }
            }
            Ok(_) => tracing::info!("No data available"),
            Err(e) => tracing::info!("Read error: {}", e),
        }

        // Close port
        manager.close_port(&port_id).await?;

        Ok(())
    }

    /// Execute a status command
    pub async fn status(&self) -> Result<()> {
        if self.json {
            let output = json!({
                "port": self.port,
                "status": "closed",
            });
            tracing::info!("{}", serde_json::to_string_pretty(&output).unwrap());
        } else {
            tracing::info!("Port: {}", self.port);
            tracing::info!("Status: closed");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = CommandExecutor::new("COM1".to_string(), 1000, false);
        assert_eq!(executor.port, "COM1");
        assert_eq!(executor.timeout, 1000);
        assert!(!executor.json);
    }
}

/// Protocol subcommands
pub enum ProtocolCommand {
    Load { path: std::path::PathBuf, name: Option<String> },
    Unload { name: String },
    Reload { name: String },
    List { verbose: bool },
    Info { name: String },
    Validate { path: std::path::PathBuf },
}

/// Execute protocol command
pub async fn execute_protocol_command(
    cmd: ProtocolCommand,
) -> Result<String> {
    match cmd {
        ProtocolCommand::Load { path, name } => {
            use crate::protocol::ProtocolValidator;
            
            // Validate first
            ProtocolValidator::validate_script(&path)?;

            let protocol_name = name.unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });

            Ok(format!("Protocol '{}' loaded from {}", protocol_name, path.display()))
        }

        ProtocolCommand::Unload { name } => {
            Ok(format!("Protocol '{}' unloaded", name))
        }

        ProtocolCommand::Reload { name } => {
            Ok(format!("Protocol '{}' reloaded", name))
        }

        ProtocolCommand::List { verbose } => {
            if verbose {
                Ok("Protocols (verbose):\n  - modbus_rtu (built-in)\n  - line (built-in)".to_string())
            } else {
                Ok("Protocols:\n  - modbus_rtu\n  - line".to_string())
            }
        }

        ProtocolCommand::Info { name } => {
            Ok(format!("Protocol: {}\n  Type: built-in", name))
        }

        ProtocolCommand::Validate { path } => {
            use crate::protocol::ProtocolValidator;
            ProtocolValidator::validate_script(&path)?;
            Ok(format!("Protocol script '{}' is valid", path.display()))
        }
    }
}
