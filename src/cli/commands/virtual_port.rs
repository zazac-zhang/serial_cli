//! Virtual serial port command handler

use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::error::{Result, SerialError};
use crate::cli::types::VirtualCommand;
use crate::serial_core::{VirtualBackend, VirtualConfig, VirtualSerialPair};

/// Global registry for active virtual port pairs
static VIRTUAL_REGISTRY: Lazy<Arc<RwLock<HashMap<String, VirtualSerialPair>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

pub async fn handle_virtual_command(cmd: VirtualCommand) -> Result<()> {
    match cmd {
        VirtualCommand::Create {
            backend,
            monitor,
            output,
            max_packets,
        } => {
            tracing::info!("Creating virtual serial port pair...");

            // Load configuration for defaults
            let config_manager = crate::config::ConfigManager::load_with_fallback();
            let app_config = config_manager.get();

            // Use backend from config if not explicitly set
            let backend_str = if backend == "pty" {
                &app_config.virtual_ports.backend
            } else {
                &backend
            };

            // Parse backend type
            let backend_type = match backend_str.as_str() {
                "pty" => VirtualBackend::Pty,
                "namedpipe" => VirtualBackend::NamedPipe,
                "socat" => VirtualBackend::Socat,
                _ => {
                    tracing::error!("Unknown backend: {}", backend_str);
                    tracing::info!("Available backends: pty, namedpipe, socat");
                    return Err(SerialError::VirtualPort(format!(
                        "Unknown backend: {}",
                        backend_str
                    )));
                }
            };

            // Use monitor from config if not explicitly set
            let monitor_enabled = if !monitor {
                app_config.virtual_ports.monitor
            } else {
                monitor
            };

            // Use max_packets from config if not explicitly set
            let max_packets_count = if max_packets == 0 {
                app_config.virtual_ports.max_packets
            } else {
                max_packets
            };

            // Use bridge_buffer_size from config
            let bridge_buffer_size_value = app_config.virtual_ports.bridge_buffer_size;

            // Create configuration
            let config = VirtualConfig {
                backend: backend_type,
                monitor: monitor_enabled,
                monitor_output: output,
                max_packets: max_packets_count,
                bridge_buffer_size: bridge_buffer_size_value,
            };

            tracing::info!("Configuration: backend={:?}, monitor={}, max_packets={}, buffer_size={}",
                config.backend, config.monitor, config.max_packets, config.bridge_buffer_size);

            // Create virtual pair
            let pair = match VirtualSerialPair::create(config).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("\u{2717} Failed to create virtual pair: {}", e);
                    return Err(e);
                }
            };

            let id = pair.id.clone();
            let port_a = pair.port_a.clone();
            let port_b = pair.port_b.clone();

            tracing::info!("\u{2713} Virtual pair created successfully");
            tracing::info!("  ID: {}", id);
            tracing::info!("  Port A: {}", port_a);
            tracing::info!("  Port B: {}", port_b);
            tracing::info!("  Backend: {:?}", pair.backend);
            tracing::info!("");
            tracing::info!("Usage examples:");
            tracing::info!("  Terminal 1: serial-cli interactive --port {}", port_a);
            tracing::info!("  Terminal 2: serial-cli interactive --port {}", port_b);
            tracing::info!("");
            tracing::info!("To stop the pair:");
            tracing::info!("  serial-cli virtual stop {}", id);

            // Store in registry
            {
                let mut registry = VIRTUAL_REGISTRY.write().await;
                registry.insert(id.clone(), pair);
            }

            // Wait for Ctrl+C to keep it running
            tracing::info!("");
            tracing::info!("Press Ctrl+C to stop the virtual pair...");

            let cleanup_result = tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("\nStopping virtual pair...");
                    let mut registry = VIRTUAL_REGISTRY.write().await;
                    if let Some(pair) = registry.remove(&id) {
                        pair.stop().await
                    } else {
                        Ok(())
                    }
                }
            };

            match cleanup_result {
                Ok(_) => tracing::info!("\u{2713} Virtual pair stopped"),
                Err(e) => {
                    tracing::error!("\u{26A0} Error during cleanup: {}", e);
                    return Err(e);
                }
            }
        }

        VirtualCommand::List => {
            let registry = VIRTUAL_REGISTRY.read().await;

            if registry.is_empty() {
                tracing::info!("No active virtual port pairs");
                tracing::info!("");
                tracing::info!("Create a virtual pair with:");
                tracing::info!("  serial-cli virtual create");
            } else {
                tracing::info!("Active virtual port pairs:");
                tracing::info!("");
                for (id, pair) in registry.iter() {
                    let stats = pair.stats().await;
                    tracing::info!("  ID: {}", id);
                    tracing::info!("    Port A: {}", stats.port_a);
                    tracing::info!("    Port B: {}", stats.port_b);
                    tracing::info!("    Backend: {:?}", stats.backend);
                    tracing::info!("    Uptime: {}s", stats.uptime_secs);
                    tracing::info!("    Status: {}", if stats.running { "Running" } else { "Stopped" });
                    tracing::info!("    Bytes bridged: {}", stats.bytes_bridged);
                    tracing::info!("    Packets bridged: {}", stats.packets_bridged);
                    if stats.bridge_errors > 0 {
                        tracing::info!("    Bridge errors: {}", stats.bridge_errors);
                    }
                    tracing::info!("");
                }
            }
        }

        VirtualCommand::Stop { id } => {
            let mut registry = VIRTUAL_REGISTRY.write().await;

            if let Some(pair) = registry.remove(&id) {
                tracing::info!("Stopping virtual pair: {}", id);
                match pair.stop().await {
                    Ok(_) => tracing::info!("\u{2713} Virtual pair stopped"),
                    Err(e) => {
                        tracing::error!("\u{26A0} Error stopping virtual pair: {}", e);
                        return Err(e);
                    }
                }
            } else {
                tracing::error!("\u{2717} Virtual pair not found: {}", id);
                tracing::info!("Use 'serial-cli virtual list' to see active pairs");
                return Err(SerialError::VirtualPort(format!(
                    "Virtual pair not found: {}",
                    id
                )));
            }
        }

        VirtualCommand::Stats { id } => {
            let registry = VIRTUAL_REGISTRY.read().await;

            if let Some(pair) = registry.get(&id) {
                let stats = pair.stats().await;
                tracing::info!("Virtual pair statistics:");
                tracing::info!("  ID: {}", stats.id);
                tracing::info!("  Port A: {}", stats.port_a);
                tracing::info!("  Port B: {}", stats.port_b);
                tracing::info!("  Backend: {:?}", stats.backend);
                tracing::info!("  Status: {}", if stats.running { "Running" } else { "Stopped" });
                tracing::info!("  Uptime: {}s", stats.uptime_secs);
                tracing::info!("  Bytes bridged: {}", stats.bytes_bridged);
                tracing::info!("  Packets bridged: {}", stats.packets_bridged);
                tracing::info!("  Bridge errors: {}", stats.bridge_errors);

                if let Some(ref error) = stats.last_error {
                    tracing::info!("  Last error: {}", error);
                }
            } else {
                tracing::error!("\u{2717} Virtual pair not found: {}", id);
                tracing::info!("Use 'serial-cli virtual list' to see active pairs");
                return Err(SerialError::VirtualPort(format!(
                    "Virtual pair not found: {}",
                    id
                )));
            }
        }
    }

    Ok(())
}
