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

    // Initialize logging - supports RUST_LOG, LOG_FORMAT, LOG_FILE env vars
    if cli.json {
        serial_cli::logging::init_json(cli.verbose);
    } else {
        serial_cli::logging::init_cli(cli.verbose);
    }

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
            let mut sniffer_config = SnifferConfig {
                max_packets,
                hex_display: display_format == "hex",
                ..SnifferConfig::default()
            };

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
                        .map_err(serial_cli::error::SerialError::Io)?;

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
            println!("Sniff statistics:");
            println!("No active sniff session");
            println!("Note: Statistics tracking requires an active sniffing session");
        }
        SniffCommand::Save { path } => {
            println!("Saving captured packets to: {}", path.display());
            println!("Note: This command requires an active sniffing session");
            println!("Use 'sniff start' to begin a sniffing session first");
        }
    }
    Ok(())
}

/// Handle batch commands
async fn handle_batch_command(cmd: BatchCommand) -> Result<()> {
    use serial_cli::cli::batch::{BatchConfig, BatchRunner};

    match cmd {
        BatchCommand::Run { script, concurrent } => {
            println!("Running batch script: {}", script.display());
            println!("Max concurrent tasks: {}", concurrent);
            println!();

            // Check if script exists
            if !script.exists() {
                println!("✗ Batch script not found: {}", script.display());
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
            if script.extension().is_some_and(|e| e == "lua") {
                // Single Lua script
                println!("Executing single script...");

                match runner.run_script(&script).await {
                    Ok(_) => {
                        println!("✓ Script executed successfully");
                    }
                    Err(e) => {
                        println!("✗ Script execution failed: {}", e);
                        return Err(e);
                    }
                }
            } else {
                // Assume it's a batch file containing list of scripts
                println!("Executing batch script file...");

                let content =
                    std::fs::read_to_string(&script).map_err(serial_cli::error::SerialError::Io)?;

                let script_paths: Vec<&std::path::Path> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| std::path::Path::new(line.trim()))
                    .collect();

                if script_paths.is_empty() {
                    println!("⚠ No scripts found in batch file");
                    return Ok(());
                }

                println!("Found {} scripts to execute", script_paths.len());

                // Run scripts in sequence
                match runner.run_scripts(script_paths).await {
                    Ok(result) => {
                        println!();
                        println!("Batch execution completed:");
                        println!("  Total scripts: {}", result.results.len());

                        let successful = result.results.iter().filter(|r| r.success).count();
                        let failed = result.results.len() - successful;

                        println!("  Successful: {}", successful);
                        println!("  Failed: {}", failed);

                        if failed > 0 {
                            println!();
                            println!("Failed scripts:");
                            for result in result.results.iter().filter(|r| !r.success) {
                                println!("  - {}", result.script);
                            }
                        }
                    }
                    Err(e) => {
                        println!("✗ Batch execution failed: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        BatchCommand::List => {
            println!("Batch scripts:");
            println!("Looking for batch scripts in current directory...");

            // List common batch script locations
            let batch_files = vec!["batch.txt", "scripts.txt", "batch.lua", "scripts.batch"];

            let mut found = false;
            for batch_file in batch_files {
                if std::path::Path::new(batch_file).exists() {
                    println!("  ✓ {}", batch_file);
                    found = true;
                }
            }

            if !found {
                println!("  No batch scripts found");
                println!();
                println!("Create a batch script file with one Lua script per line:");
                println!("  # Comments start with #");
                println!("  script1.lua");
                println!("  script2.lua");
                println!("  script3.lua");
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
                // Output JSON data directly to stdout
                println!("{}", serde_json::to_string_pretty(&config).unwrap());
            } else {
                // Output configuration display directly to stdout
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
                println!("  file = \"{}\"", config.logging.file);
                println!();
                println!("[lua]");
                println!("  memory_limit_mb = {}", config.lua.memory_limit_mb);
                println!("  timeout_seconds = {}", config.lua.timeout_seconds);
                println!("  enable_sandbox = {}", config.lua.enable_sandbox);
                println!();
                println!("[task]");
                println!("  max_concurrent = {}", config.task.max_concurrent);
                println!(
                    "  default_timeout_seconds = {}",
                    config.task.default_timeout_seconds
                );
                println!();
                println!("[output]");
                println!("  json_pretty = {}", config.output.json_pretty);
                println!("  show_timestamp = {}", config.output.show_timestamp);
                println!();
                println!("Use 'config set <key> <value>' to modify configuration");
                println!("Use 'config save [path]' to save configuration to file");
                println!("Use 'config reset' to reset to defaults");
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
                    println!("✗ Failed to set configuration: {}", e);
                    println!();
                    println!("Valid configuration keys:");
                    println!("  serial.baudrate              - Baud rate (e.g., 115200)");
                    println!("  serial.databits              - Data bits (5-8)");
                    println!("  serial.stopbits              - Stop bits (1-2)");
                    println!("  serial.parity                - Parity (none/odd/even)");
                    println!("  serial.timeout_ms            - Timeout in milliseconds");
                    println!(
                        "  logging.level                - Log level (error/warn/info/debug/trace)"
                    );
                    println!("  logging.format               - Log format (text/json)");
                    println!("  logging.file                 - Log file path");
                    println!("  lua.memory_limit_mb          - Lua memory limit");
                    println!("  lua.timeout_seconds          - Lua timeout");
                    println!("  lua.enable_sandbox           - Enable Lua sandbox");
                    println!("  task.max_concurrent          - Max concurrent tasks");
                    println!("  task.default_timeout_seconds - Default task timeout");
                    println!("  output.json_pretty           - Pretty print JSON");
                    println!("  output.show_timestamp        - Show timestamps");
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

    // Output JSON data directly to stdout (not through logging)
    println!("{}", serde_json::to_string_pretty(&output).unwrap());

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
