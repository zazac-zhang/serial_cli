//! CLI argument definitions
//!
//! Top-level CLI parser and command routing.

use clap::{Parser, Subcommand};

use super::types::{BatchCommand, ConfigCommand, ProtocolCommand, SniffCommand, VirtualCommand};

#[derive(Parser)]
#[command(name = "serial-cli")]
#[command(about = "A universal serial port CLI tool optimized for AI interaction", long_about = None)]
pub struct Cli {
    /// Enable JSON output
    #[arg(long, global = true)]
    pub json: bool,

    /// Verbose mode
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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

    /// Virtual serial port commands
    Virtual {
        #[command(subcommand)]
        virtual_command: VirtualCommand,
    },

    /// (Internal) Background sniff daemon — not for direct user invocation
    #[command(hide = true, name = "__sniff_daemon__")]
    SniffDaemon {
        #[arg(long)]
        port: String,

        #[arg(long)]
        output: Option<String>,

        #[arg(long, default_value = "0")]
        max_packets: usize,

        #[arg(long, default_value = "false")]
        hex: bool,
    },
}
