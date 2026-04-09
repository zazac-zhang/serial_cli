//! Interactive shell
//!
//! This module provides an interactive REPL shell for serial communication.

use crate::error::{Result, SerialError};
use crate::serial_core::{PortManager, SerialConfig};
use std::io::{self, Write};

/// Interactive shell
pub struct InteractiveShell {
    running: bool,
    manager: PortManager,
    current_port_id: Option<String>,
}

impl InteractiveShell {
    /// Create a new interactive shell
    pub fn new() -> Self {
        Self {
            running: false,
            manager: PortManager::new(),
            current_port_id: None,
        }
    }

    /// Run the interactive shell
    pub async fn run(&mut self) -> Result<()> {
        self.running = true;
        tracing::info!("Serial CLI Interactive Shell");
        tracing::info!("Type 'help' for available commands, 'quit' to exit");
        tracing::info!("");

        while self.running {
            tracing::trace!("serial> ");
            io::stdout().flush().map_err(SerialError::Io)?;

            let mut line = String::new();
            io::stdin().read_line(&mut line).map_err(SerialError::Io)?;

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Err(e) = self.execute_command(line).await {
                tracing::info!("Error: {}", e);
            }
        }

        Ok(())
    }

    /// Execute a command
    async fn execute_command(&mut self, line: &str) -> Result<()> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            "help" => self.cmd_help(),
            "list" => self.cmd_list().await?,
            "open" => self.cmd_open(&parts[1..]).await?,
            "close" => self.cmd_close(&parts[1..]).await?,
            "send" => self.cmd_send(&parts[1..]).await?,
            "recv" => self.cmd_recv(&parts[1..]).await?,
            "status" => self.cmd_status().await?,
            "protocol" => self.cmd_protocol(&parts[1..]).await?,
            "dtr" => self.cmd_dtr(&parts[1..]).await?,
            "rts" => self.cmd_rts(&parts[1..]).await?,
            "quit" | "exit" => {
                tracing::info!("Goodbye!");
                self.running = false;
            }
            _ => tracing::info!(
                "Unknown command: {}. Type 'help' for available commands.",
                parts[0]
            ),
        }

        Ok(())
    }

    /// Help command
    fn cmd_help(&self) {
        tracing::info!("Available commands:");
        tracing::info!("  help              - Show this help message");
        tracing::info!("  list              - List available serial ports");
        tracing::info!("  open <port>       - Open a serial port");
        tracing::info!("  close [port_id]   - Close a serial port (closes current if no ID given)");
        tracing::info!("  send <data>       - Send data to the current port");
        tracing::info!("  recv [n]          - Receive data from the current port (default: 64 bytes)");
        tracing::info!("  status            - Show port status");
        tracing::info!("");
        tracing::info!("Protocol commands:");
        tracing::info!("  protocol          - Show current protocol and available protocols");
        tracing::info!("  protocol list     - List all available protocols");
        tracing::info!("  protocol set <name>  - Set protocol for current port");
        tracing::info!("  protocol clear    - Clear protocol from current port");
        tracing::info!("  protocol show     - Show protocol status");
        tracing::info!("");
        tracing::info!("Hardware control commands:");
        tracing::info!("  dtr [on|off]      - Get or set DTR signal state");
        tracing::info!("  rts [on|off]      - Get or set RTS signal state");
        tracing::info!("");
        tracing::info!("  quit/exit         - Exit the shell");
    }

    /// List ports command
    async fn cmd_list(&self) -> Result<()> {
        let ports = self.manager.list_ports()?;

        if ports.is_empty() {
            tracing::info!("No serial ports found.");
        } else {
            tracing::info!("Available serial ports:");
            for port in ports {
                tracing::info!("  - {} ({})", port.port_name, port.port_type);
            }
        }

        Ok(())
    }

    /// Open port command
    async fn cmd_open(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            tracing::info!("Usage: open <port>");
            return Ok(());
        }

        let port_name = args[0];

        // Close current port if open
        if let Some(ref port_id) = self.current_port_id {
            tracing::info!("Closing current port...");
            self.manager.close_port(port_id).await?;
            self.current_port_id = None;
        }

        tracing::info!("Opening port: {}", port_name);

        // Use default configuration
        let config = SerialConfig::default();

        // Open the port
        match self.manager.open_port(port_name, config).await {
            Ok(port_id) => {
                tracing::info!("Port opened successfully");
                tracing::info!("Port ID: {}", port_id);
                self.current_port_id = Some(port_id);
            }
            Err(e) => {
                tracing::info!("Failed to open port: {}", e);
            }
        }

        Ok(())
    }

    /// Close port command
    async fn cmd_close(&mut self, args: &[&str]) -> Result<()> {
        let port_id = if args.is_empty() {
            // Use current port
            if let Some(ref id) = self.current_port_id {
                id.clone()
            } else {
                tracing::info!("No port is currently open");
                tracing::info!("Usage: close <port_id>");
                return Ok(());
            }
        } else {
            // Use specified port
            args[0].to_string()
        };

        match self.manager.close_port(&port_id).await {
            Ok(_) => {
                tracing::info!("Port closed successfully");
                if self.current_port_id.as_ref() == Some(&port_id) {
                    self.current_port_id = None;
                }
            }
            Err(e) => {
                tracing::info!("Failed to close port: {}", e);
            }
        }

        Ok(())
    }

    /// Send command
    async fn cmd_send(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            tracing::info!("Usage: send <data>");
            return Ok(());
        }

        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        let data = args.join(" ");
        let port_id = self.current_port_id.as_ref().unwrap();

        tracing::info!("Sending: {}", data);

        // Get the port handle
        let port_handle = self.manager.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;

        // Send data
        match handle.write(data.as_bytes()) {
            Ok(n) => {
                tracing::info!("Sent {} bytes", n);
            }
            Err(e) => {
                tracing::info!("Failed to send data: {}", e);
            }
        }

        Ok(())
    }

    /// Receive command
    async fn cmd_recv(&mut self, args: &[&str]) -> Result<()> {
        let n: usize = if args.is_empty() {
            64
        } else {
            args[0].parse().unwrap_or(64)
        };

        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        tracing::info!("Reading up to {} bytes...", n);

        let port_id = self.current_port_id.as_ref().unwrap();

        // Get the port handle
        let port_handle = self.manager.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;

        // Read data
        let mut buffer = vec![0u8; n];
        match handle.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    buffer.truncate(bytes_read);

                    // Try to display as string
                    if let Ok(text) = String::from_utf8(buffer.clone()) {
                        tracing::info!("Received ({} bytes as text): {}", bytes_read, text);
                    } else {
                        // Display as hex
                        let hex: String = buffer.iter().map(|b| format!("{:02x} ", b)).collect();
                        tracing::info!("Received ({} bytes as hex): {}", bytes_read, hex);
                    }
                } else {
                    tracing::info!("No data available");
                }
            }
            Err(e) => {
                tracing::info!("Failed to read data: {}", e);
            }
        }

        Ok(())
    }

    /// Status command
    async fn cmd_status(&self) -> Result<()> {
        if let Some(ref port_id) = self.current_port_id {
            tracing::info!("Current port ID: {}", port_id);

            // Try to get port info
            match self.manager.get_port(port_id).await {
                Ok(port_handle) => {
                    let handle = port_handle.lock().await;
                    tracing::info!("Port name: {}", handle.name());
                    tracing::info!("Configuration:");
                    tracing::info!("  Baud rate: {}", handle.config().baudrate);
                    tracing::info!("  Data bits: {}", handle.config().databits);
                    tracing::info!("  Stop bits: {}", handle.config().stopbits);
                    tracing::info!("  Parity: {:?}", handle.config().parity);
                    tracing::info!("  Flow control: {:?}", handle.config().flow_control);

                    // Show protocol information
                    match handle.protocol() {
                        Some(protocol) => tracing::info!("  Protocol: {}", protocol),
                        None => tracing::info!("  Protocol: (none - raw mode)"),
                    }
                }
                Err(_) => {
                    tracing::info!("Port handle not available");
                }
            }
        } else {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' to open a port");
        }

        Ok(())
    }

    /// Protocol command
    async fn cmd_protocol(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            // Show current protocol and available protocols
            self.show_protocol_status().await?;
        } else {
            match args[0] {
                "list" | "ls" => {
                    // List all available protocols
                    self.list_protocols().await?;
                }
                "set" => {
                    // Set protocol for current port
                    if args.len() < 2 {
                        tracing::info!("Usage: protocol set <protocol_name>");
                        tracing::info!("Available protocols:");
                        self.list_protocols().await?;
                    } else {
                        self.set_port_protocol(args[1]).await?;
                    }
                }
                "show" | "status" => {
                    // Show current protocol details
                    self.show_protocol_status().await?;
                }
                "clear" | "none" => {
                    // Clear protocol from current port
                    self.clear_port_protocol().await?;
                }
                _ => {
                    // Try to set protocol directly (shorthand)
                    self.set_port_protocol(args[0]).await?;
                }
            }
        }

        Ok(())
    }

    /// Show protocol status for current port
    async fn show_protocol_status(&self) -> Result<()> {
        if let Some(ref port_id) = self.current_port_id {
            match self.manager.get_port_protocol(port_id).await {
                Ok(Some(protocol)) => {
                    tracing::info!("Current protocol: {}", protocol);
                    tracing::info!("");
                    tracing::info!("Protocol commands:");
                    tracing::info!("  protocol list          - List all available protocols");
                    tracing::info!("  protocol set <name>    - Set protocol for current port");
                    tracing::info!("  protocol clear         - Clear protocol from current port");
                    tracing::info!("  protocol show          - Show protocol status");
                }
                Ok(None) => {
                    tracing::info!("Current protocol: (none)");
                    tracing::info!("");
                    tracing::info!("Available protocols:");
                    self.list_protocols().await?;
                    tracing::info!("");
                    tracing::info!("Use 'protocol set <name>' to attach a protocol to this port");
                }
                Err(e) => {
                    tracing::info!("Error getting protocol: {}", e);
                }
            }
        } else {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
        }

        Ok(())
    }

    /// List all available protocols
    async fn list_protocols(&self) -> Result<()> {
        tracing::info!("Built-in protocols:");
        tracing::info!("  - modbus_rtu      - Modbus RTU protocol");
        tracing::info!("  - modbus_ascii    - Modbus ASCII protocol");
        tracing::info!("  - at_command      - AT Command protocol");
        tracing::info!("  - line            - Line-based protocol");
        tracing::info!("");
        tracing::info!("Custom protocols can be loaded with 'protocol_load' in Lua scripts");

        Ok(())
    }

    /// Set protocol for current port
    async fn set_port_protocol(&mut self, protocol_name: &str) -> Result<()> {
        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        // Validate protocol name
        let valid_protocols = vec!["modbus_rtu", "modbus_ascii", "at_command", "line"];
        if !valid_protocols.contains(&protocol_name) {
            tracing::info!("Unknown protocol: {}", protocol_name);
            tracing::info!("");
            tracing::info!("Available protocols:");
            self.list_protocols().await?;
            return Ok(());
        }

        let port_id = self.current_port_id.as_ref().unwrap();
        match self
            .manager
            .set_port_protocol(port_id, Some(protocol_name.to_string()))
            .await
        {
            Ok(_) => {
                tracing::info!("Protocol '{}' set for port", protocol_name);
                tracing::info!("Data will now be processed using the {} protocol", protocol_name);
            }
            Err(e) => {
                tracing::info!("Failed to set protocol: {}", e);
            }
        }

        Ok(())
    }

    /// Clear protocol from current port
    async fn clear_port_protocol(&mut self) -> Result<()> {
        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        let port_id = self.current_port_id.as_ref().unwrap();
        match self
            .manager
            .set_port_protocol(port_id, None)
            .await
        {
            Ok(_) => {
                tracing::info!("Protocol cleared from port");
                tracing::info!("Data will be processed as raw bytes");
            }
            Err(e) => {
                tracing::info!("Failed to clear protocol: {}", e);
            }
        }

        Ok(())
    }

    /// DTR command
    async fn cmd_dtr(&mut self, args: &[&str]) -> Result<()> {
        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        if args.is_empty() {
            // Show current DTR state
            let port_id = self.current_port_id.as_ref().unwrap();
            match self.manager.get_dtr(port_id).await {
                Ok(state) => tracing::info!("DTR signal: {}", if state { "ON" } else { "OFF" }),
                Err(e) => tracing::info!("Error getting DTR state: {}", e),
            }
            tracing::info!("");
            tracing::info!("Usage: dtr on|off");
            return Ok(());
        }

        let enable = match args[0].to_lowercase().as_str() {
            "on" | "true" | "1" | "enable" => true,
            "off" | "false" | "0" | "disable" => false,
            _ => {
                tracing::info!("Invalid argument: {}", args[0]);
                tracing::info!("Usage: dtr on|off");
                return Ok(());
            }
        };

        let port_id = self.current_port_id.as_ref().unwrap();
        match self.manager.set_dtr(port_id, enable).await {
            Ok(_) => {
                tracing::info!("DTR signal set to: {}", if enable { "ON" } else { "OFF" });
                tracing::info!("Note: Full platform-specific DTR control implementation pending");
            }
            Err(e) => {
                tracing::info!("Failed to set DTR: {}", e);
            }
        }

        Ok(())
    }

    /// RTS command
    async fn cmd_rts(&mut self, args: &[&str]) -> Result<()> {
        if self.current_port_id.is_none() {
            tracing::info!("No port is currently open");
            tracing::info!("Use 'open <port>' first");
            return Ok(());
        }

        if args.is_empty() {
            // Show current RTS state
            let port_id = self.current_port_id.as_ref().unwrap();
            match self.manager.get_rts(port_id).await {
                Ok(state) => tracing::info!("RTS signal: {}", if state { "ON" } else { "OFF" }),
                Err(e) => tracing::info!("Error getting RTS state: {}", e),
            }
            tracing::info!("");
            tracing::info!("Usage: rts on|off");
            return Ok(());
        }

        let enable = match args[0].to_lowercase().as_str() {
            "on" | "true" | "1" | "enable" => true,
            "off" | "false" | "0" | "disable" => false,
            _ => {
                tracing::info!("Invalid argument: {}", args[0]);
                tracing::info!("Usage: rts on|off");
                return Ok(());
            }
        };

        let port_id = self.current_port_id.as_ref().unwrap();
        match self.manager.set_rts(port_id, enable).await {
            Ok(_) => {
                tracing::info!("RTS signal set to: {}", if enable { "ON" } else { "OFF" });
                tracing::info!("Note: Full platform-specific RTS control implementation pending");
            }
            Err(e) => {
                tracing::info!("Failed to set RTS: {}", e);
            }
        }

        Ok(())
    }
}

impl Default for InteractiveShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_creation() {
        let shell = InteractiveShell::new();
        assert!(!shell.running);
        assert!(shell.current_port_id.is_none());
    }
}
