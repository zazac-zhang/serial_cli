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

    manager
        .open_port(&port_name, core_config)
        .await
        .map_err(|e: serial_cli::error::SerialError| e.to_string())
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
