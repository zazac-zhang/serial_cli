// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::state::app_state::AppState;
use crate::state::port_state::{PortStatus, SerialConfig, PortStats};
use serial_cli::serial_core::{PortManager, SerialConfig as CoreSerialConfig, Parity, FlowControl};
use tauri::State;

/// List available serial ports
#[tauri::command]
pub async fn list_ports(state: State<'_, AppState>) -> Result<Vec<PortInfo>, String> {
    use tokio::sync::MutexGuard;

    let manager: MutexGuard<serial_cli::serial_core::PortManager> = state.port_manager.lock().await;
    manager
        .list_ports()
        .map(|ports| {
            ports
                .into_iter()
                .map(|p| PortInfo {
                    port_name: p.port_name,
                    port_type: format!("{:?}", p.port_type),
                })
                .collect()
        })
        .map_err(|e| e.to_string())
}

/// Open a serial port
#[tauri::command]
pub async fn open_port(
    port_name: String,
    config: SerialConfig,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use tokio::sync::MutexGuard;

    let manager: MutexGuard<PortManager> = state.port_manager.lock().await;

    // Convert UI config to core config
    let core_config = CoreSerialConfig {
        baudrate: config.baudrate,
        databits: config.databits,
        stopbits: config.stopbits,
        parity: parse_parity(&config.parity),
        timeout_ms: config.timeout_ms,
        flow_control: parse_flow_control(&config.flow_control),
        dtr_enable: false,
        rts_enable: false,
    };

    let port_id = manager
        .open_port(&port_name, core_config)
        .await
        .map_err(|e: serial_cli::error::SerialError| e.to_string())?;

    // Spawn background task to read data from this port
    let port_manager_clone = state.port_manager.clone();
    let port_id_clone = port_id.clone();
    let app_handle = app.clone();

    tauri::async_runtime::spawn(async move {
        let mut buffer = vec![0u8; 4096];

        loop {
            // Try to get the port
            let manager = port_manager_clone.lock().await;
            let port_handle = match manager.get_port(&port_id_clone).await {
                Ok(handle) => handle,
                Err(_) => {
                    // Port was closed
                    break;
                }
            };
            drop(manager);

            // Try to read data
            let mut handle = port_handle.lock().await;
            match handle.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    buffer.truncate(n);
                    let data = buffer.clone();

                    // Emit data-received event
                    if let Err(e) = crate::events::emitter::emit_data_received(
                        app_handle.clone(),
                        port_id_clone.clone(),
                        data,
                    ).await {
                        eprintln!("Failed to emit data-received event: {}", e);
                    }
                }
                Ok(_) => {
                    // No data available
                }
                Err(_) => {
                    // Port error or closed
                    break;
                }
            }

            // Small delay to prevent busy-waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });

    Ok(port_id)
}

/// Close a serial port
#[tauri::command]
pub async fn close_port(port_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let manager = state.port_manager.lock().await;
    manager
        .close_port(&port_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get port status
#[tauri::command]
pub async fn get_port_status(
    port_id: String,
    state: State<'_, AppState>,
) -> Result<PortStatus, String> {
    use tokio::sync::MutexGuard;

    let manager: MutexGuard<PortManager> = state.port_manager.lock().await;
    let port_handle = manager
        .get_port(&port_id)
        .await
        .map_err(|e: serial_cli::error::SerialError| e.to_string())?;
    let handle = port_handle.lock().await;

    Ok(PortStatus {
        id: port_id,
        port_name: handle.name().to_string(),
        is_open: true,
        config: Some(SerialConfig {
            baudrate: handle.config().baudrate,
            databits: handle.config().databits,
            stopbits: handle.config().stopbits,
            parity: format!("{:?}", handle.config().parity),
            timeout_ms: handle.config().timeout_ms,
            flow_control: format!("{:?}", handle.config().flow_control),
        }),
        stats: PortStats::default(),
    })
}

/// Port information
#[derive(serde::Serialize)]
pub struct PortInfo {
    pub port_name: String,
    pub port_type: String,
}

/// Parse parity from string
fn parse_parity(parity: &str) -> Parity {
    match parity.to_lowercase().as_str() {
        "odd" => Parity::Odd,
        "even" => Parity::Even,
        _ => Parity::None,
    }
}

/// Parse flow control from string
fn parse_flow_control(flow: &str) -> FlowControl {
    match flow.to_lowercase().as_str() {
        "software" => FlowControl::Software,
        "hardware" => FlowControl::Hardware,
        _ => FlowControl::None,
    }
}
