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

        /// Arguments to pass to the script
        #[arg(value_name = "ARGS", trailing_var_arg = true)]
        args: Vec<String>,
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

        /// Enable real-time display
        #[arg(long, default_value = "true")]
        display: bool,

        /// Display format (hex/raw)
        #[arg(long, default_value = "hex")]
        format: String,
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

async fn run_lua_script(path: PathBuf, args: Vec<String>) -> Result<()> {
    use serial_cli::lua::executor::ScriptEngine;

    // 1. Create script engine
    let engine = ScriptEngine::new()?;

    // 2. Register all available APIs
    engine.bindings.register_all_apis()?;

    // 3. Register stdlib utilities
    register_stdlib_utils(&engine.bindings)?;

    // 4. Read script file
    let script_content =
        std::fs::read_to_string(&path).map_err(serial_cli::error::SerialError::Io)?;

    // 5. Execute script with arguments
    if args.is_empty() {
        engine.execute_file(&path)?;
    } else {
        tracing::info!("Executing script with arguments: {:?}", args);
        engine.execute_with_args(&script_content, args)?;
    }

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
    let config_manager = serial_cli::config::ConfigManager::load_with_fallback();
    let _config = config_manager.get();

    // Validate configuration
    if let Err(e) = config_manager.validate() {
        tracing::info!("Warning: Configuration validation failed: {}", e);
    }

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
        Commands::Run { script, args } => {
            run_lua_script(PathBuf::from(script), args).await?;
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
            tracing::info!("Available protocols:");
            if verbose {
                tracing::info!("Built-in protocols:");
                tracing::info!("  modbus_rtu      - Modbus RTU protocol (Industrial communication)");
                tracing::info!("  modbus_ascii    - Modbus ASCII protocol (Industrial communication)");
                tracing::info!("  at_command      - AT Command protocol (Modem control)");
                tracing::info!("  line            - Line-based protocol (Text-based communication)");
            } else {
                tracing::info!("  modbus_rtu");
                tracing::info!("  modbus_ascii");
                tracing::info!("  at_command");
                tracing::info!("  line");
            }
            tracing::info!("");
            tracing::info!("Custom protocols can be loaded via Lua scripts");
        }
        ProtocolCommand::Info { name } => {
            tracing::info!("Protocol: {}", name);
            let descriptions = vec![
                ("modbus_rtu", "Modbus RTU protocol - Binary industrial communication protocol"),
                ("modbus_ascii", "Modbus ASCII protocol - Text-based industrial communication protocol"),
                ("at_command", "AT Command protocol - Modem control commands"),
                ("line", "Line-based protocol - Simple text line communication"),
            ];

            if let Some((_, desc)) = descriptions.iter().find(|(n, _)| *n == name) {
                tracing::info!("Description: {}", desc);
            } else {
                tracing::info!("Description: Custom protocol");
            }
        }
        ProtocolCommand::Validate { path } => {
            use serial_cli::protocol::ProtocolValidator;
            tracing::info!("Validating protocol script: {}", path.display());
            match ProtocolValidator::validate_script(&path) {
                Ok(_) => tracing::info!("✓ Protocol script is valid"),
                Err(e) => {
                    tracing::info!("✗ Validation failed: {}", e);
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

/// Handle sniff commands
async fn handle_sniff_command(cmd: SniffCommand) -> Result<()> {
    use serial_cli::serial_core::{SerialSniffer, SnifferConfig};

    match cmd {
        SniffCommand::Start {
            port,
            output,
            max_packets,
            display,
            format: display_format,
        } => {
            tracing::info!("Starting sniff on port: {}", port);
            tracing::info!("Max packets: {}", max_packets);
            let display_str = if display { "enabled" } else { "disabled" };
            tracing::info!("Real-time display: {}", display_str);
            tracing::info!("Display format: {}", display_format);
            if let Some(ref out_path) = output {
                tracing::info!("Output file: {}", out_path.display());
            }
            tracing::info!("");

            // Create sniffer configuration
            let mut sniffer_config = SnifferConfig::default();
            sniffer_config.max_packets = max_packets;
            sniffer_config.hex_display = display_format == "hex";

            if output.is_some() {
                sniffer_config.save_to_file = true;
                if let Some(ref out_path) = output {
                    if let Some(parent) = out_path.parent() {
                        sniffer_config.output_dir = parent.to_path_buf();
                    }
                }
            }

            // Create sniffer
            let sniffer = SerialSniffer::new(sniffer_config.clone());

            // Start sniffing
            match sniffer.start_sniffing(&port).await {
                Ok(_session) => {
                    tracing::info!("✓ Sniffing started successfully on port: {}", port);
                    if display {
                        tracing::info!("Real-time display enabled - Press Ctrl+C to stop");
                        tracing::info!("");
                    } else {
                        tracing::info!("Press Ctrl+C to stop sniffing");
                    }

                    // Keep sniffing until interrupted
                    tokio::signal::ctrl_c()
                        .await
                        .map_err(|e| serial_cli::error::SerialError::Io(e))?;

                    tracing::info!("\nStopping sniff...");

                    // Get packet statistics
                    let packet_count = sniffer.packet_count().await;
                    tracing::info!("Captured {} packets", packet_count);

                    // Save to file if requested
                    if let Some(out_path) = output {
                        tracing::info!("Saving to: {}", out_path.display());
                        if let Err(e) = sniffer.save_to_file(&out_path).await {
                            tracing::info!("Warning: Failed to save: {}", e);
                        } else {
                            tracing::info!("✓ Saved successfully");
                        }
                    }
                }
                Err(e) => {
                    tracing::info!("✗ Failed to start sniffing: {}", e);
                    return Err(e);
                }
            }
        }
        SniffCommand::Stats => {
            tracing::info!("Sniff statistics:");
            tracing::info!("No active sniff session");
            tracing::info!("Note: Statistics tracking requires an active sniffing session");
        }
        SniffCommand::Save { path } => {
            tracing::info!("Saving captured packets to: {}", path.display());
            tracing::info!("Note: This command requires an active sniffing session");
            tracing::info!("Use 'sniff start' to begin a sniffing session first");
        }
    }
    Ok(())
}

/// Handle batch commands
async fn handle_batch_command(cmd: BatchCommand) -> Result<()> {
    use serial_cli::cli::batch::{BatchConfig, BatchRunner};

    match cmd {
        BatchCommand::Run { script, concurrent } => {
            tracing::info!("Running batch script: {}", script.display());
            tracing::info!("Max concurrent tasks: {}", concurrent);
            tracing::info!("");

            // Check if script exists
            if !script.exists() {
                tracing::info!("✗ Batch script not found: {}", script.display());
                return Err(serial_cli::error::SerialError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Batch script not found",
                )));
            }

            // Create batch configuration
            let config = BatchConfig {
                max_concurrent: concurrent,
                timeout_secs: 300, // 5 minutes default
                continue_on_error: false,
                show_progress: true,
                verbose: false,
            };

            // Create batch runner
            let runner = BatchRunner::new(config)?;

            // Check if it's a single script or a batch file
            if script.extension().map_or(false, |e| e == "lua") {
                // Single Lua script
                tracing::info!("Executing single script...");

                match runner.run_script(&script).await {
                    Ok(_) => {
                        tracing::info!("✓ Script executed successfully");
                    }
                    Err(e) => {
                        tracing::info!("✗ Script execution failed: {}", e);
                        return Err(e);
                    }
                }
            } else {
                // Assume it's a batch file containing list of scripts
                tracing::info!("Executing batch script file...");

                let content = std::fs::read_to_string(&script)
                    .map_err(serial_cli::error::SerialError::Io)?;

                let script_paths: Vec<&std::path::Path> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| std::path::Path::new(line.trim()))
                    .collect();

                if script_paths.is_empty() {
                    tracing::info!("⚠ No scripts found in batch file");
                    return Ok(());
                }

                tracing::info!("Found {} scripts to execute", script_paths.len());

                // Run scripts in sequence
                match runner.run_scripts(script_paths).await {
                    Ok(result) => {
                        tracing::info!("");
                        tracing::info!("Batch execution completed:");
                        tracing::info!("  Total scripts: {}", result.results.len());

                        let successful = result.results.iter().filter(|r| r.success).count();
                        let failed = result.results.len() - successful;

                        tracing::info!("  Successful: {}", successful);
                        tracing::info!("  Failed: {}", failed);

                        if failed > 0 {
                            tracing::info!("");
                            tracing::info!("Failed scripts:");
                            for result in result.results.iter().filter(|r| !r.success) {
                                tracing::info!("  - {}", result.script);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::info!("✗ Batch execution failed: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        BatchCommand::List => {
            tracing::info!("Batch scripts:");
            tracing::info!("Looking for batch scripts in current directory...");

            // List common batch script locations
            let batch_files = vec!["batch.txt", "scripts.txt", "batch.lua", "scripts.batch"];

            let mut found = false;
            for batch_file in batch_files {
                if std::path::Path::new(batch_file).exists() {
                    tracing::info!("  ✓ {}", batch_file);
                    found = true;
                }
            }

            if !found {
                tracing::info!("  No batch scripts found");
                tracing::info!("");
                tracing::info!("Create a batch script file with one Lua script per line:");
                tracing::info!("  # Comments start with #");
                tracing::info!("  script1.lua");
                tracing::info!("  script2.lua");
                tracing::info!("  script3.lua");
            }
        }
    }
    Ok(())
}

/// Handle config commands
fn handle_config_command(cmd: ConfigCommand) -> Result<()> {
    use serial_cli::config::ConfigManager;

    let config_manager = ConfigManager::load_with_fallback();

    match cmd {
        ConfigCommand::Show { json } => {
            let config = config_manager.get();
            if json {
                tracing::info!("{}", serde_json::to_string_pretty(&config).unwrap());
            } else {
                tracing::info!("Current configuration:");
                tracing::info!("");
                tracing::info!("[serial]");
                tracing::info!("  baudrate = {}", config.serial.baudrate);
                tracing::info!("  databits = {}", config.serial.databits);
                tracing::info!("  stopbits = {}", config.serial.stopbits);
                tracing::info!("  parity = \"{}\"", config.serial.parity);
                tracing::info!("  timeout_ms = {}", config.serial.timeout_ms);
                tracing::info!("");
                tracing::info!("[logging]");
                tracing::info!("  level = \"{}\"", config.logging.level);
                tracing::info!("  format = \"{}\"", config.logging.format);
                tracing::info!("  file = \"{}\"", config.logging.file);
                tracing::info!("");
                tracing::info!("[lua]");
                tracing::info!("  memory_limit_mb = {}", config.lua.memory_limit_mb);
                tracing::info!("  timeout_seconds = {}", config.lua.timeout_seconds);
                tracing::info!("  enable_sandbox = {}", config.lua.enable_sandbox);
                tracing::info!("");
                tracing::info!("[task]");
                tracing::info!("  max_concurrent = {}", config.task.max_concurrent);
                tracing::info!("  default_timeout_seconds = {}", config.task.default_timeout_seconds);
                tracing::info!("");
                tracing::info!("[output]");
                tracing::info!("  json_pretty = {}", config.output.json_pretty);
                tracing::info!("  show_timestamp = {}", config.output.show_timestamp);
                tracing::info!("");
                tracing::info!("Use 'config set <key> <value>' to modify configuration");
                tracing::info!("Use 'config save [path]' to save configuration to file");
                tracing::info!("Use 'config reset' to reset to defaults");
            }
        }
        ConfigCommand::Set { key, value } => {
            tracing::info!("Setting configuration: {} = {}", key, value);

            match config_manager.set(&key, &value) {
                Ok(_) => {
                    tracing::info!("✓ Configuration updated successfully");

                    // Validate after setting
                    if let Err(e) = config_manager.validate() {
                        tracing::info!("⚠ Warning: Configuration may be invalid: {}", e);
                    }

                    tracing::info!("Note: Use 'config save' to persist changes");
                }
                Err(e) => {
                    tracing::info!("✗ Failed to set configuration: {}", e);
                    tracing::info!("");
                    tracing::info!("Valid configuration keys:");
                    tracing::info!("  serial.baudrate              - Baud rate (e.g., 115200)");
                    tracing::info!("  serial.databits              - Data bits (5-8)");
                    tracing::info!("  serial.stopbits              - Stop bits (1-2)");
                    tracing::info!("  serial.parity                - Parity (none/odd/even)");
                    tracing::info!("  serial.timeout_ms            - Timeout in milliseconds");
                    tracing::info!("  logging.level                - Log level (error/warn/info/debug/trace)");
                    tracing::info!("  logging.format               - Log format (text/json)");
                    tracing::info!("  logging.file                 - Log file path");
                    tracing::info!("  lua.memory_limit_mb          - Lua memory limit");
                    tracing::info!("  lua.timeout_seconds          - Lua timeout");
                    tracing::info!("  lua.enable_sandbox           - Enable Lua sandbox");
                    tracing::info!("  task.max_concurrent          - Max concurrent tasks");
                    tracing::info!("  task.default_timeout_seconds - Default task timeout");
                    tracing::info!("  output.json_pretty           - Pretty print JSON");
                    tracing::info!("  output.show_timestamp        - Show timestamps");
                    return Err(e);
                }
            }
        }
        ConfigCommand::Save { path } => {
            let output_path = path.unwrap_or_else(|| {
                if let Some(global_path) = serial_cli::config::get_global_config_path() {
                    global_path
                } else {
                    std::path::PathBuf::from(".serial-cli.toml")
                }
            });

            tracing::info!("Saving configuration to: {}", output_path.display());

            match config_manager.save(Some(&output_path)) {
                Ok(_) => {
                    tracing::info!("✓ Configuration saved successfully");
                }
                Err(e) => {
                    tracing::info!("✗ Failed to save configuration: {}", e);
                    return Err(e);
                }
            }
        }
        ConfigCommand::Reset => {
            tracing::info!("Resetting configuration to defaults...");

            match config_manager.reset() {
                Ok(_) => {
                    tracing::info!("✓ Configuration reset to defaults");
                    tracing::info!("Note: Use 'config save' to persist changes");
                }
                Err(e) => {
                    tracing::info!("✗ Failed to reset configuration: {}", e);
                    return Err(e);
                }
            }
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

    tracing::info!("{}", serde_json::to_string_pretty(&output).unwrap());

    Ok(())
}

async fn send_data(port: &str, data: &str) -> Result<()> {
    use serial_cli::serial_core::{PortManager, SerialConfig};
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
                tracing::info!("Received response ({} bytes): {}", bytes_read, response);
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
