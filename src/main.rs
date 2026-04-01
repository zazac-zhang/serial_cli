//! Serial CLI - Main entry point

use clap::{Parser, Subcommand};
use serial_cli::error::Result;
use serial_cli::cli::interactive::InteractiveShell;
use tracing_subscriber;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

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
            println!("Running script: {} - coming soon!", script);
        }
    }

    Ok(())
}

fn list_ports() -> Result<()> {
    use serial_cli::serial_core::PortManager;
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

async fn send_data(port: &str, data: &str) -> Result<()> {
    println!("Sending '{}' to port {}", data, port);
    // TODO: Implement actual sending
    Ok(())
}
