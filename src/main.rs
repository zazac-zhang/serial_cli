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

    /// Protocol management commands
    Protocol {
        #[command(subcommand)]
        protocol_command: ProtocolCommand,
    },

    /// Sniff/monitor serial port traffic
    Sniff {
        #[command(subcommand)]
        sniff_command: SniffCommand,
    },

    /// Batch execution commands
    Batch {
        #[command(subcommand)]
        batch_command: BatchCommand,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        config_command: ConfigCommand,
    },
}

/// Protocol subcommands
#[derive(Subcommand)]
enum ProtocolCommand {
    /// List all available protocols
    List {
        /// Show verbose information
        #[arg(long)]
        verbose: bool,
    },

    /// Show protocol information
    Info {
        /// Protocol name
        name: String,
    },

    /// Validate a protocol script
    Validate {
        /// Path to protocol script
        path: PathBuf,
    },
}

/// Sniff subcommands
#[derive(Subcommand)]
enum SniffCommand {
    /// Start sniffing on a port
    Start {
        /// Port name
        #[arg(short, long)]
        port: String,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum packets to capture
        #[arg(short, long, default_value = "1000")]
        max_packets: usize,
    },

    /// Show sniffing statistics
    Stats,

    /// Save captured packets to file
    Save {
        /// Output file path
        path: PathBuf,
    },
}

/// Batch subcommands
#[derive(Subcommand)]
enum BatchCommand {
    /// Run a batch script
    Run {
        /// Path to batch script
        script: PathBuf,

        /// Maximum concurrent tasks
        #[arg(short, long, default_value = "5")]
        concurrent: usize,
    },

    /// List batch scripts
    List,
}

/// Config subcommands
#[derive(Subcommand)]
enum ConfigCommand {
    /// Show current configuration
    Show {
        /// Show as JSON
        #[arg(long)]
        json: bool,
    },

    /// Set a configuration value
    Set {
        /// Configuration key (e.g., serial.baudrate)
        key: String,

        /// Configuration value
        value: String,
    },

    /// Save configuration to file
    Save {
        /// Output file path (optional)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Reset configuration to defaults
    Reset,
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
        Commands::Protocol { protocol_command } => {
            handle_protocol_command(protocol_command)?;
        }
        Commands::Sniff { sniff_command } => {
            handle_sniff_command(sniff_command).await?;
        }
        Commands::Batch { batch_command } => {
            handle_batch_command(batch_command).await?;
        }
        Commands::Config { config_command } => {
            handle_config_command(config_command)?;
        }
    }

    Ok(())
}

/// Handle protocol commands
fn handle_protocol_command(cmd: ProtocolCommand) -> Result<()> {
    match cmd {
        ProtocolCommand::List { verbose } => {
            println!("Available protocols:");
            if verbose {
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
            let descriptions = vec![
                ("modbus_rtu", "Modbus RTU protocol - Binary industrial communication protocol"),
                ("modbus_ascii", "Modbus ASCII protocol - Text-based industrial communication protocol"),
                ("at_command", "AT Command protocol - Modem control commands"),
                ("line", "Line-based protocol - Simple text line communication"),
            ];

            if let Some((_, desc)) = descriptions.iter().find(|(n, _)| *n == name) {
                println!("Description: {}", desc);
            } else {
                println!("Description: Custom protocol");
            }
        }
        ProtocolCommand::Validate { path } => {
            use serial_cli::protocol::ProtocolValidator;
            println!("Validating protocol script: {}", path.display());
            match ProtocolValidator::validate_script(&path) {
                Ok(_) => println!("✓ Protocol script is valid"),
                Err(e) => {
                    println!("✗ Validation failed: {}", e);
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

/// Handle sniff commands
async fn handle_sniff_command(cmd: SniffCommand) -> Result<()> {
    match cmd {
        SniffCommand::Start { port, output, max_packets } => {
            println!("Starting sniff on port: {}", port);
            println!("Max packets: {}", max_packets);
            if let Some(out_path) = output {
                println!("Output file: {}", out_path.display());
            }
            println!();
            println!("Note: Sniff functionality requires full implementation");
            println!("The IoLoop and Sniffer modules are implemented but need integration");
        }
        SniffCommand::Stats => {
            println!("Sniff statistics:");
            println!("No active sniff session");
        }
        SniffCommand::Save { path } => {
            println!("Saving captured packets to: {}", path.display());
            println!("Note: Save functionality requires full implementation");
        }
    }
    Ok(())
}

/// Handle batch commands
async fn handle_batch_command(cmd: BatchCommand) -> Result<()> {
    match cmd {
        BatchCommand::Run { script, concurrent } => {
            println!("Running batch script: {}", script.display());
            println!("Max concurrent tasks: {}", concurrent);
            println!();
            println!("Note: Batch functionality is partially implemented");
            println!("The BatchRunner module exists but needs CLI integration");
        }
        BatchCommand::List => {
            println!("Batch scripts:");
            println!("No batch scripts found");
            println!("Note: Batch script management needs implementation");
        }
    }
    Ok(())
}

/// Handle config commands
fn handle_config_command(cmd: ConfigCommand) -> Result<()> {
    match cmd {
        ConfigCommand::Show { json } => {
            let config = serial_cli::config::Config::default();
            if json {
                println!("{}", serde_json::to_string_pretty(&config).unwrap());
            } else {
                println!("Current configuration:");
                println!();
                println!("[serial]");
                println!("  baudrate = {}", config.serial.baudrate);
                println!("  databits = {}", config.serial.databits);
                println!("  stopbits = {}", config.serial.stopbits);
                println!("  parity = \"{}\"", config.serial.parity);
                println!("  timeout_ms = {}", config.serial.timeout_ms);
                println!();
                println!("[logging]");
                println!("  level = \"{}\"", config.logging.level);
                println!("  format = \"{}\"", config.logging.format);
                println!();
                println!("Note: Use 'config set <key> <value>' to modify configuration");
            }
        }
        ConfigCommand::Set { key, value } => {
            println!("Setting configuration:");
            println!("  {} = {}", key, value);
            println!();
            println!("Note: Configuration setting needs implementation");
            println!("The config module supports loading, but runtime setting is not complete");
        }
        ConfigCommand::Save { path } => {
            let output_path = path.unwrap_or_else(|| std::path::PathBuf::from(".serial-cli.toml"));
            println!("Saving configuration to: {}", output_path.display());
            println!();
            println!("Note: Configuration saving needs implementation");
        }
        ConfigCommand::Reset => {
            println!("Resetting configuration to defaults");
            println!();
            println!("Note: Configuration reset needs implementation");
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
