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
        println!("Serial CLI Interactive Shell");
        println!("Type 'help' for available commands, 'quit' to exit");
        println!();

        while self.running {
            print!("serial> ");
            io::stdout().flush()
                .map_err(|e| SerialError::Io(e))?;

            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .map_err(|e| SerialError::Io(e))?;

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Err(e) = self.execute_command(line).await {
                eprintln!("Error: {}", e);
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
            "quit" | "exit" => {
                println!("Goodbye!");
                self.running = false;
            }
            _ => println!("Unknown command: {}. Type 'help' for available commands.", parts[0]),
        }

        Ok(())
    }

    /// Help command
    fn cmd_help(&self) {
        println!("Available commands:");
        println!("  help              - Show this help message");
        println!("  list              - List available serial ports");
        println!("  open <port>       - Open a serial port");
        println!("  close [port_id]   - Close a serial port (closes current if no ID given)");
        println!("  send <data>       - Send data to the current port");
        println!("  recv [n]          - Receive data from the current port (default: 64 bytes)");
        println!("  status            - Show port status");
        println!("  protocol <name>   - Set protocol for current port");
        println!("  quit/exit         - Exit the shell");
    }

    /// List ports command
    async fn cmd_list(&self) -> Result<()> {
        let ports = self.manager.list_ports()?;

        if ports.is_empty() {
            println!("No serial ports found.");
        } else {
            println!("Available serial ports:");
            for port in ports {
                println!("  - {} ({})", port.port_name, port.port_type);
            }
        }

        Ok(())
    }

    /// Open port command
    async fn cmd_open(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: open <port>");
            return Ok(());
        }

        let port_name = args[0];

        // Close current port if open
        if let Some(ref port_id) = self.current_port_id {
            println!("Closing current port...");
            self.manager.close_port(port_id).await?;
            self.current_port_id = None;
        }

        println!("Opening port: {}", port_name);

        // Use default configuration
        let config = SerialConfig::default();

        // Open the port
        match self.manager.open_port(port_name, config).await {
            Ok(port_id) => {
                println!("Port opened successfully");
                println!("Port ID: {}", port_id);
                self.current_port_id = Some(port_id);
            }
            Err(e) => {
                println!("Failed to open port: {}", e);
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
                println!("No port is currently open");
                println!("Usage: close <port_id>");
                return Ok(());
            }
        } else {
            // Use specified port
            args[0].to_string()
        };

        match self.manager.close_port(&port_id).await {
            Ok(_) => {
                println!("Port closed successfully");
                if self.current_port_id.as_ref() == Some(&port_id) {
                    self.current_port_id = None;
                }
            }
            Err(e) => {
                println!("Failed to close port: {}", e);
            }
        }

        Ok(())
    }

    /// Send command
    async fn cmd_send(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: send <data>");
            return Ok(());
        }

        if self.current_port_id.is_none() {
            println!("No port is currently open");
            println!("Use 'open <port>' first");
            return Ok(());
        }

        let data = args.join(" ");
        let port_id = self.current_port_id.as_ref().unwrap();

        println!("Sending: {}", data);

        // Get the port handle
        let port_handle = self.manager.get_port(port_id).await?;
        let mut handle = port_handle.lock().await;

        // Send data
        match handle.write(data.as_bytes()) {
            Ok(n) => {
                println!("Sent {} bytes", n);
            }
            Err(e) => {
                println!("Failed to send data: {}", e);
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
            println!("No port is currently open");
            println!("Use 'open <port>' first");
            return Ok(());
        }

        println!("Reading up to {} bytes...", n);

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
                        println!("Received ({} bytes as text): {}", bytes_read, text);
                    } else {
                        // Display as hex
                        let hex: String = buffer.iter()
                            .map(|b| format!("{:02x} ", b))
                            .collect();
                        println!("Received ({} bytes as hex): {}", bytes_read, hex);
                    }
                } else {
                    println!("No data available");
                }
            }
            Err(e) => {
                println!("Failed to read data: {}", e);
            }
        }

        Ok(())
    }

    /// Status command
    async fn cmd_status(&self) -> Result<()> {
        if let Some(ref port_id) = self.current_port_id {
            println!("Current port ID: {}", port_id);

            // Try to get port info
            match self.manager.get_port(port_id).await {
                Ok(port_handle) => {
                    let handle = port_handle.lock().await;
                    println!("Port name: {}", handle.name());
                    println!("Configuration:");
                    println!("  Baud rate: {}", handle.config().baudrate);
                    println!("  Data bits: {}", handle.config().databits);
                    println!("  Stop bits: {}", handle.config().stopbits);
                    println!("  Parity: {:?}", handle.config().parity);
                }
                Err(_) => {
                    println!("Port handle not available");
                }
            }
        } else {
            println!("No port is currently open");
            println!("Use 'open <port>' to open a port");
        }

        Ok(())
    }

    /// Protocol command
    async fn cmd_protocol(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Current protocol: (none - protocol management coming soon)");
            println!("Available protocols:");
            println!("  - modbus_rtu");
            println!("  - at_command");
            println!("  - line");
            println!();
            println!("Protocol support will be implemented in the next update");
        } else {
            println!("Protocol management coming soon");
            println!("Requested protocol: {}", args[0]);
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
        assert_eq!(shell.running, false);
        assert!(shell.current_port_id.is_none());
    }
}
