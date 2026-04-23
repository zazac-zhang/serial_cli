//! Serial CLI - Main entry point

use std::path::PathBuf;

use clap::Parser;

use serial_cli::cli::args::{Cli, Commands};
use serial_cli::cli::interactive::InteractiveShell;
use serial_cli::cli::commands::{
    batch as batch_cmd, config as config_cmd, ports, protocol as protocol_cmd,
    script, sniff as sniff_cmd, virtual_port,
};
use serial_cli::cli::sniff_session;
use serial_cli::error::Result;

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
        Some(Commands::ListPorts) => {
            ports::list_ports()?;
        }
        Some(Commands::Send { port, data }) => {
            ports::send_data(&port, &data).await?;
        }
        Some(Commands::Interactive) => {
            let mut shell = InteractiveShell::new();
            shell.run().await?;
        }
        Some(Commands::Run { script, args }) => {
            script::run_lua_script(PathBuf::from(script), args).await?;
        }
        Some(Commands::Protocol { protocol_command }) => {
            protocol_cmd::handle_protocol_command(protocol_command)?;
        }
        Some(Commands::Sniff { sniff_command }) => {
            sniff_cmd::handle_sniff_command(sniff_command).await?;
        }
        Some(Commands::Batch { batch_command }) => {
            batch_cmd::handle_batch_command(batch_command).await?;
        }
        Some(Commands::Config { config_command }) => {
            config_cmd::handle_config_command(config_command)?;
        }
        Some(Commands::Virtual { virtual_command }) => {
            virtual_port::handle_virtual_command(virtual_command).await?;
        }
        Some(Commands::SniffDaemon {
            port,
            output,
            max_packets,
            hex,
        }) => {
            sniff_session::run_sniff_daemon(&port, output.as_deref().map(std::path::Path::new), max_packets, hex)
                .await?;
        }
        None => {
            // No command specified, default to interactive mode
            let mut shell = InteractiveShell::new();
            shell.run().await?;
        }
    }

    Ok(())
}
