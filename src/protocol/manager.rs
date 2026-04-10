//! Protocol manager for custom protocol lifecycle
//!
//! Manages loading, unloading, reloading, and persistence of custom protocols.

use crate::error::{ProtocolError, Result, SerialError};
use crate::protocol::loader::ProtocolLoader;
use crate::protocol::validator::ProtocolValidator;
use crate::protocol::{ProtocolInfo, ProtocolRegistry};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Custom protocol metadata
#[derive(Debug, Clone)]
pub struct CustomProtocol {
    pub name: String,
    pub script_path: PathBuf,
    pub loaded_at: std::time::SystemTime,
    pub version: u64,
}

/// Protocol manager
pub struct ProtocolManager {
    registry: Arc<Mutex<ProtocolRegistry>>,
    custom_protocols: HashMap<String, CustomProtocol>,
}

impl ProtocolManager {
    /// Create a new protocol manager
    pub fn new(registry: Arc<Mutex<ProtocolRegistry>>) -> Self {
        Self {
            registry,
            custom_protocols: HashMap::new(),
        }
    }

    /// Load a protocol from a file
    pub async fn load_protocol(&mut self, path: &Path) -> Result<ProtocolInfo> {
        // Load the protocol
        let loaded = ProtocolLoader::load_from_file(path)?;

        // Create factory
        let factory = ProtocolLoader::create_factory(&loaded)?;

        // Store metadata
        let custom = CustomProtocol {
            name: loaded.name.clone(),
            script_path: loaded.script_path.clone(),
            loaded_at: loaded.loaded_at,
            version: 1,
        };

        // Register to protocol registry
        let mut registry = self.registry.lock().await;
        registry.register(factory).await;
        drop(registry);

        // Store in custom protocols tracking
        self.custom_protocols.insert(loaded.name.clone(), custom);

        tracing::info!(
            "Successfully loaded and registered protocol: {}",
            loaded.name
        );

        Ok(ProtocolInfo {
            name: loaded.name,
            description: "Custom Lua protocol".to_string(),
        })
    }

    /// Unload a protocol
    pub async fn unload_protocol(&mut self, name: &str) -> Result<()> {
        // Check if protocol exists
        if !self.custom_protocols.contains_key(name) {
            return Err(SerialError::Protocol(ProtocolError::NotFound(
                name.to_string(),
            )));
        }

        // Remove from metadata
        self.custom_protocols.remove(name);

        // Remove from registry
        let mut registry = self.registry.lock().await;
        registry.unregister(name).await?;

        tracing::info!("Successfully unloaded protocol: {}", name);

        Ok(())
    }

    /// Reload a protocol
    pub async fn reload_protocol(&mut self, name: &str) -> Result<()> {
        // Get existing metadata
        let script_path = self
            .custom_protocols
            .get(name)
            .ok_or_else(|| SerialError::Protocol(ProtocolError::NotFound(name.to_string())))?
            .script_path
            .clone();

        // Unload existing
        self.unload_protocol(name).await?;

        // Reload from file
        self.load_protocol(&script_path).await?;

        Ok(())
    }

    /// List all protocols (built-in + custom)
    pub async fn list_protocols(&self) -> Vec<ProtocolInfo> {
        let registry = self.registry.lock().await;
        let mut protocols = registry.list_protocols().await;

        // Mark custom protocols
        for proto in protocols.iter_mut() {
            if self.custom_protocols.contains_key(&proto.name) {
                proto.description = format!("{} (custom)", proto.description);
            }
        }

        protocols
    }

    /// Get custom protocol metadata
    pub fn get_custom_protocol(&self, name: &str) -> Option<&CustomProtocol> {
        self.custom_protocols.get(name)
    }

    /// Get number of custom protocols
    pub fn custom_protocols_len(&self) -> usize {
        self.custom_protocols.len()
    }

    /// Validate a protocol script without loading
    pub fn validate_protocol(path: &Path) -> Result<()> {
        ProtocolValidator::validate_script(path)?;
        Ok(())
    }
}
