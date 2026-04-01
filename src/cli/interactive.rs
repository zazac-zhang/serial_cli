//! Interactive shell
//!
//! This module provides an interactive REPL shell for serial communication.

use crate::error::{Result, SerialError};
use std::io::{self, Write};

/// Interactive shell
pub struct InteractiveShell {
    running: bool,
}

impl InteractiveShell {
    /// Create a new interactive shell
    pub fn new() -> Self {
        Self { running: false }
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
        println!("  close [port_id]   - Close a serial port");
        println!("  send <data>       - Send data to the current port");
        println!("  recv [n]          - Receive data from the current port");
        println!("  status            - Show port status");
        println!("  protocol <name>   - Set protocol for current port");
        println!("  quit/exit         - Exit the shell");
    }

    /// List ports command
    async fn cmd_list(&self) -> Result<()> {
        use crate::serial_core::PortManager;
        let manager = PortManager::new();
        let ports = manager.list_ports()?;

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
    async fn cmd_open(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: open <port>");
            return Ok(());
        }

        println!("Opening port: {} (not yet implemented)", args[0]);
        // TODO: Implement actual port opening
        Ok(())
    }

    /// Close port command
    async fn cmd_close(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Closing current port (not yet implemented)");
        } else {
            println!("Closing port: {} (not yet implemented)", args[0]);
        }
        // TODO: Implement actual port closing
        Ok(())
    }

    /// Send command
    async fn cmd_send(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Usage: send <data>");
            return Ok(());
        }

        let data = args.join(" ");
        println!("Sending: {} (not yet implemented)", data);
        // TODO: Implement actual send
        Ok(())
    }

    /// Receive command
    async fn cmd_recv(&self, args: &[&str]) -> Result<()> {
        let n: usize = if args.is_empty() {
            64
        } else {
            args[0].parse().unwrap_or(64)
        };

        println!("Receiving {} bytes (not yet implemented)", n);
        // TODO: Implement actual recv
        Ok(())
    }

    /// Status command
    async fn cmd_status(&self) -> Result<()> {
        println!("Port status: (not yet implemented)");
        // TODO: Implement actual status
        Ok(())
    }

    /// Protocol command
    async fn cmd_protocol(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Current protocol: (none)");
            println!("Available protocols: modbus_rtu, at_command, line");
        } else {
            println!("Setting protocol to: {} (not yet implemented)", args[0]);
        }
        // TODO: Implement actual protocol setting
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
    }
}
