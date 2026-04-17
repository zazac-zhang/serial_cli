//! CLI command type definitions
//!
//! This module contains all command and subcommand enum definitions.

use std::path::PathBuf;

/// Protocol subcommands
#[derive(clap::Subcommand)]
pub enum ProtocolCommand {
    /// List all available protocols
    List {
        /// Show verbose information including descriptions
        #[arg(long)]
        detailed: bool,
    },

    /// Show protocol information
    Info {
        /// Protocol name
        name: String,
    },

    /// Load a custom protocol from Lua script
    Load {
        /// Path to protocol script
        path: PathBuf,

        /// Custom protocol name (default: filename without extension)
        #[arg(long)]
        name: Option<String>,
    },

    /// Unload a custom protocol
    Unload {
        /// Protocol name
        name: String,
    },

    /// Reload a custom protocol from disk
    Reload {
        /// Protocol name
        name: String,
    },

    /// Validate a protocol script without loading
    Validate {
        /// Path to protocol script
        path: PathBuf,
    },
}

/// Sniff subcommands
#[derive(clap::Subcommand)]
pub enum SniffCommand {
    /// Start sniffing on a port
    Start {
        /// Port name
        #[arg(short, long)]
        port: String,

        /// Output file path (optional, auto-generated if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum packets to capture (0 = unlimited)
        #[arg(short, long, default_value = "0")]
        max_packets: usize,

        /// Enable real-time display
        #[arg(long, default_value = "true")]
        display: bool,

        /// Display format (raw or hex)
        #[arg(long, default_value = "raw")]
        format: String,
    },

    /// Stop sniffing
    Stop,

    /// Show sniffing statistics
    Stats,

    /// Save captured packets to file
    Save {
        /// Output file path
        #[arg(short, long)]
        path: PathBuf,
    },
}

/// Batch subcommands
#[derive(clap::Subcommand)]
pub enum BatchCommand {
    /// Run batch processing
    Run {
        /// Script or batch file path
        script: PathBuf,

        /// Maximum concurrent tasks
        #[arg(long, default_value = "5")]
        concurrent: usize,

        /// Continue on error
        #[arg(long)]
        continue_on_error: bool,

        /// Task timeout in seconds
        #[arg(long, default_value = "60")]
        timeout: u64,
    },

    /// List batch files
    List,
}

/// Config subcommands
#[derive(clap::Subcommand)]
pub enum ConfigCommand {
    /// Show configuration
    Show {
        /// Show as JSON
        #[arg(long)]
        json: bool,
    },

    /// Set a configuration value
    Set {
        /// Configuration key (e.g., serial.baudrate, logging.level)
        key: String,

        /// Configuration value
        value: String,
    },

    /// Save configuration to file
    Save {
        /// Output file path (optional, uses default if not specified)
        #[arg(long)]
        path: Option<PathBuf>,
    },

    /// Reset configuration to defaults
    Reset,
}
