//! Serial CLI - Main entry point

use clap::{Parser, Subcommand};
use serial_cli::cli::interactive::InteractiveShell;
use serial_cli::error::Result;
use serial_cli::lua::bindings::LuaBindings;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "serial-cli")]
#[command(about = "A universal serial port CLI tool optimized for AI interaction", long_about = None)]
struct Cli {
    /// Enable JSON output
    #[arg(long, global = true)]
    json: bool,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available serial ports
    ListPorts,

    /// Send data to a serial port
    Send {
        /// Port name (e.g., COM1, /dev/ttyUSB0)
        #[arg(short, long)]
        port: String,

        /// Data to send
        data: String,
    },

    /// Interactive shell mode
    Interactive,

    /// Run a Lua script
    Run {
        /// Script file to run
        script: String,
    },
}

async fn run_lua_script(path: PathBuf) -> Result<()> {
    use serial_cli::lua::bindings::LuaBindings;

    // 1. Create Lua bindings
    let bindings = LuaBindings::new()?;

    // 2. Register all available APIs (log, utility, serial, protocol)
    bindings.register_all_apis()?;

    // 3. Register stdlib utilities manually to the same Lua instance
    register_stdlib_utils(&bindings)?;

    // 4. Read and execute script file
    let script_content =
        std::fs::read_to_string(&path).map_err(serial_cli::error::SerialError::Io)?;

    bindings.execute_script(&script_content)?;

    Ok(())
}

/// Register stdlib utility functions to the bindings' Lua instance
fn register_stdlib_utils(bindings: &LuaBindings) -> Result<()> {
    use mlua::Value;
    let lua = bindings.lua();
    let globals = lua.globals();

    // string_to_hex
    let to_hex = lua.create_function(|_, data: String| {
        Ok(data
            .bytes()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
    })?;
    globals.set("string_to_hex", to_hex)?;

    // string_from_hex
    let from_hex = lua.create_function(|_, hex: String| {
        if !hex.len().is_multiple_of(2) {
            return Err(mlua::Error::RuntimeError(
                "Hex string must have even length".to_string(),
            ));
        }

        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| mlua::Error::RuntimeError("Invalid hex string".to_string()))?;
            bytes.push(byte);
        }

        String::from_utf8(bytes).map_err(|_| mlua::Error::RuntimeError("Invalid UTF-8".to_string()))
    })?;
    globals.set("string_from_hex", from_hex)?;

    // hex_encode
    let encode = lua.create_function(|_, data: Vec<u8>| {
        Ok(data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
    })?;
    globals.set("hex_encode", encode)?;

    // hex_decode
    let decode = lua.create_function(|_, hex: String| {
        if !hex.len().is_multiple_of(2) {
            return Err(mlua::Error::RuntimeError(
                "Hex string must have even length".to_string(),
            ));
        }

        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| mlua::Error::RuntimeError("Invalid hex string".to_string()))?;
            bytes.push(byte);
        }

        Ok(bytes)
    })?;
    globals.set("hex_decode", decode)?;

    // hex_to_bytes
    let hex_to_bytes = lua.create_function(|lua: &mlua::Lua, hex: String| {
        if !hex.len().is_multiple_of(2) {
            return Err(mlua::Error::RuntimeError(
                "Hex string must have even length".to_string(),
            ));
        }

        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| mlua::Error::RuntimeError(format!("Invalid hex: {}", byte_str)))?;
            bytes.push(byte);
        }

        let result = lua.create_table()?;
        for (i, byte) in bytes.iter().enumerate() {
            result.set(i + 1, *byte)?;
        }
        Ok(result)
    })?;
    globals.set("hex_to_bytes", hex_to_bytes)?;

    // bytes_to_hex
    let bytes_to_hex = lua.create_function(|_, bytes: Value| {
        let bytes_vec = match bytes {
            Value::String(s) => {
                let s = s.to_str().unwrap();
                s.as_bytes().to_vec()
            }
            Value::Table(t) => {
                let mut vec = Vec::new();
                for pair in t.pairs::<usize, u8>() {
                    let (_, byte) = pair.unwrap();
                    vec.push(byte);
                }
                vec
            }
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Expected string or table".to_string(),
                ))
            }
        };

        let hex: String = bytes_vec.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(hex)
    })?;
    globals.set("bytes_to_hex", bytes_to_hex)?;

    // bytes_to_string
    let bytes_to_string = lua.create_function(|_, bytes: Value| {
        let bytes_vec = match bytes {
            Value::Table(t) => {
                let mut vec = Vec::new();
                for pair in t.pairs::<usize, u8>() {
                    let (_, byte) = pair.unwrap();
                    vec.push(byte);
                }
                vec
            }
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Expected table of bytes".to_string(),
                ))
            }
        };

        String::from_utf8(bytes_vec)
            .map_err(|_| mlua::Error::RuntimeError("Invalid UTF-8 sequence".to_string()))
    })?;
    globals.set("bytes_to_string", bytes_to_string)?;

    // string_to_bytes
    let string_to_bytes = lua.create_function(|lua: &mlua::Lua, s: String| {
        let bytes = s.into_bytes();
        let result = lua.create_table()?;
        for (i, byte) in bytes.iter().enumerate() {
            result.set(i + 1, *byte)?;
        }
        Ok(result)
    })?;
    globals.set("string_to_bytes", string_to_bytes)?;

    // time_now
    let now = lua.create_function(|_, _: ()| {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0))
    })?;
    globals.set("time_now", now)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(log_level).init();

    // Load configuration
    let _config = serial_cli::config::load_config_with_fallback();

    // Execute command
    match cli.command {
        Commands::ListPorts => {
            list_ports()?;
        }
        Commands::Send { port, data } => {
            send_data(&port, &data).await?;
        }
        Commands::Interactive => {
            let mut shell = InteractiveShell::new();
            shell.run().await?;
        }
        Commands::Run { script } => {
            run_lua_script(PathBuf::from(script)).await?;
        }
    }

    Ok(())
}

fn list_ports() -> Result<()> {
    use serde_json::json;
    use serial_cli::serial_core::PortManager;

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

async fn send_data(port: &str, data: &str) -> Result<()> {
    use serial_cli::serial_core::{PortManager, SerialConfig};
    use std::thread;
    use std::time::Duration;

    println!("Opening port: {}", port);

    // Create port manager
    let manager = PortManager::new();

    // Use default configuration
    let config = SerialConfig::default();

    // Open the port
    let port_id = manager.open_port(port, config).await?;

    println!("Port opened successfully: {}", port_id);
    println!("Sending data: {}", data);

    // Get the port handle
    let port_handle = manager.get_port(&port_id).await?;
    let mut handle = port_handle.lock().await;

    // Convert data to bytes
    let bytes = data.as_bytes();

    // Send data
    let bytes_written = handle.write(bytes)?;
    println!("Sent {} bytes", bytes_written);

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
                println!("No response received");
            }
        }
        Err(e) => {
            println!("Note: Could not read response: {}", e);
        }
    }

    // Close the port
    drop(handle);
    manager.close_port(&port_id).await?;
    println!("Port closed");

    Ok(())
}
