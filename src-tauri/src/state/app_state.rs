// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serial_cli::protocol::{ProtocolManager, ProtocolRegistry};
use serial_cli::serial_core::PortManager;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global application state shared across all Tauri commands
#[derive(Clone)]
pub struct AppState {
    /// Serial port manager
    pub port_manager: Arc<Mutex<PortManager>>,
    /// Protocol registry
    pub protocol_registry: Arc<Mutex<ProtocolRegistry>>,
    /// Protocol manager for custom protocols
    pub protocol_manager: Arc<Mutex<ProtocolManager>>,
}

impl AppState {
    /// Create a new application state
    pub async fn new() -> Self {
        let protocol_registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
        let protocol_manager = Arc::new(Mutex::new(ProtocolManager::new(protocol_registry.clone())));

        Self {
            port_manager: Arc::new(Mutex::new(PortManager::new())),
            protocol_registry,
            protocol_manager,
        }
    }
}
