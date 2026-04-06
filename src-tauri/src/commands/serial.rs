// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::state::app_state::AppState;
use tauri::State;

/// Send data to a serial port
#[tauri::command]
pub async fn send_data(
    port_id: String,
    data: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    use tokio::sync::MutexGuard;

    let manager: MutexGuard<serial_cli::serial_core::PortManager> = state.port_manager.lock().await;
    let port_handle = manager
        .get_port(&port_id)
        .await
        .map_err(|e: serial_cli::error::SerialError| e.to_string())?;
    let mut handle = port_handle.lock().await;
    handle.write(&data).map_err(|e: serial_cli::error::SerialError| e.to_string())
}

/// Read data from a serial port
#[tauri::command]
pub async fn read_data(
    port_id: String,
    max_bytes: usize,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    use tokio::sync::MutexGuard;

    let manager: MutexGuard<serial_cli::serial_core::PortManager> = state.port_manager.lock().await;
    let port_handle = manager
        .get_port(&port_id)
        .await
        .map_err(|e: serial_cli::error::SerialError| e.to_string())?;
    let mut handle = port_handle.lock().await;
    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = handle.read(&mut buffer).map_err(|e: serial_cli::error::SerialError| e.to_string())?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Start sniffing data on a port
#[tauri::command]
pub async fn start_sniffing(
    port_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implement sniffing
    Ok(())
}

/// Stop sniffing data on a port
#[tauri::command]
pub async fn stop_sniffing(
    port_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implement sniffing
    Ok(())
}
