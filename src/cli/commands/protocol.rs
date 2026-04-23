//! Protocol command handler

use crate::error::Result;
use crate::cli::types::ProtocolCommand;

pub fn handle_protocol_command(cmd: ProtocolCommand) -> Result<()> {
    match cmd {
        ProtocolCommand::List { detailed } => {
            println!("Available protocols:");
            if detailed {
                println!("Built-in protocols:");
                println!("  modbus_rtu      - Modbus RTU protocol (Industrial communication)");
                println!("  modbus_ascii    - Modbus ASCII protocol (Industrial communication)");
                println!("  at_command      - AT Command protocol (Modem control)");
                println!("  line            - Line-based protocol (Text-based communication)");
            } else {
                println!("  modbus_rtu");
                println!("  modbus_ascii");
                println!("  at_command");
                println!("  line");
            }
            println!();
            println!("Custom protocols can be loaded via Lua scripts");
        }
        ProtocolCommand::Info { name } => {
            println!("Protocol: {}", name);
            let descriptions = [
                (
                    "modbus_rtu",
                    "Modbus RTU protocol - Binary industrial communication protocol",
                ),
                (
                    "modbus_ascii",
                    "Modbus ASCII protocol - Text-based industrial communication protocol",
                ),
                ("at_command", "AT Command protocol - Modem control commands"),
                (
                    "line",
                    "Line-based protocol - Simple text line communication",
                ),
            ];

            if let Some((_, desc)) = descriptions.iter().find(|(n, _)| *n == name) {
                println!("Description: {}", desc);
            } else {
                println!("Description: Custom protocol");
            }
        }
        ProtocolCommand::Validate { path } => {
            use crate::protocol::ProtocolValidator;
            println!("Validating protocol script: {}", path.display());
            match ProtocolValidator::validate_script(&path) {
                Ok(_) => println!("\u{2713} Protocol script is valid"),
                Err(e) => {
                    println!("\u{2717} Validation failed: {}", e);
                    return Err(e);
                }
            }
        }
        ProtocolCommand::Load { path, name } => {
            let display_name = name.as_deref().unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
            });
            println!("Loading protocol: {} from {}", display_name, path.display());
            println!("Note: Dynamic protocol loading will be implemented in a future version");
        }
        ProtocolCommand::Unload { name } => {
            println!("Unloading protocol: {}", name);
            println!("Note: Dynamic protocol unloading will be implemented in a future version");
        }
        ProtocolCommand::Reload { name } => {
            println!("Reloading protocol: {}", name);
            println!("Note: Dynamic protocol reloading will be implemented in a future version");
        }
    }
    Ok(())
}
