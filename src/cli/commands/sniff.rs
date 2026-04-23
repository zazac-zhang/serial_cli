//! Sniff command handler

use crate::error::Result;
use crate::cli::types::SniffCommand;
use crate::serial_core::{SerialSniffer, SnifferConfig};

pub async fn handle_sniff_command(cmd: SniffCommand) -> Result<()> {
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
                    tracing::info!("\u{2713} Sniffing started successfully on port: {}", port);
                    if display {
                        tracing::info!("Real-time display enabled - Press Ctrl+C to stop");
                        tracing::info!("");
                    } else {
                        tracing::info!("Press Ctrl+C to stop sniffing");
                    }

                    // Keep sniffing until interrupted
                    tokio::signal::ctrl_c()
                        .await
                        .map_err(crate::error::SerialError::Io)?;

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
                            tracing::info!("\u{2713} Saved successfully");
                        }
                    }
                }
                Err(e) => {
                    tracing::info!("\u{2717} Failed to start sniffing: {}", e);
                    return Err(e);
                }
            }
        }
        SniffCommand::Stop => {
            println!("Stopping active sniff session...");
            println!("Note: Session tracking will be implemented in a future version");
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
