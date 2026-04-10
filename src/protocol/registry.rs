//! Protocol registry and management
//!
//! This module provides protocol registration, discovery, and lifecycle management.

use crate::error::{ProtocolError, Result, SerialError};
use crate::protocol::Protocol;
use std::collections::HashMap;
use std::sync::Arc;

/// Protocol factory trait for creating protocol instances
pub trait ProtocolFactory: Send + Sync {
    /// Create a new protocol instance
    fn create(&self) -> Result<Box<dyn Protocol>>;

    /// Get the protocol name
    fn name(&self) -> &str;

    /// Get the protocol description
    fn description(&self) -> &str {
        ""
    }
}

/// Protocol registry for managing available protocols
pub struct ProtocolRegistry {
    factories: HashMap<String, Arc<dyn ProtocolFactory>>,
}

impl ProtocolRegistry {
    /// Create a new protocol registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// Register a protocol factory
    pub async fn register(&mut self, factory: Arc<dyn ProtocolFactory>) {
        let name = factory.name().to_string();
        self.factories.insert(name, factory);
    }

    /// Get or create a protocol instance
    pub async fn get_protocol(&self, name: &str) -> Result<Box<dyn Protocol>> {
        // Create new instance from factory
        let factory = self
            .factories
            .get(name)
            .ok_or_else(|| SerialError::Protocol(ProtocolError::NotFound(name.to_string())))?;

        let protocol = factory.create()?;
        Ok(protocol)
    }

    /// List all registered protocols
    pub async fn list_protocols(&self) -> Vec<ProtocolInfo> {
        self.factories
            .iter()
            .map(|(name, factory)| ProtocolInfo {
                name: name.clone(),
                description: factory.description().to_string(),
            })
            .collect()
    }

    /// Clear all protocol instances (no-op in new implementation)
    pub async fn clear_instances(&self) {
        // No longer needed with factory pattern
    }

    /// Unregister a protocol by name
    pub async fn unregister(&mut self, name: &str) -> Result<()> {
        if self.factories.remove(name).is_none() {
            return Err(SerialError::Protocol(ProtocolError::NotFound(
                name.to_string(),
            )));
        }
        tracing::info!("Unregistered protocol: {}", name);
        Ok(())
    }

    /// Check if a protocol is registered
    pub async fn is_registered(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

impl Default for ProtocolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Protocol information
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtocolInfo {
    pub name: String,
    pub description: String,
}

/// Simple protocol factory implementation
pub struct SimpleProtocolFactory<P, F>
where
    P: Protocol + Clone + 'static,
    F: Fn() -> P + Send + Sync,
{
    name: String,
    description: String,
    creator: F,
    _phantom: std::marker::PhantomData<P>,
}

impl<P, F> SimpleProtocolFactory<P, F>
where
    P: Protocol + Clone + 'static,
    F: Fn() -> P + Send + Sync,
{
    pub fn new(name: String, description: String, creator: F) -> Self {
        Self {
            name,
            description,
            creator,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<P, F> ProtocolFactory for SimpleProtocolFactory<P, F>
where
    P: Protocol + Clone + 'static,
    F: Fn() -> P + Send + Sync,
{
    fn create(&self) -> Result<Box<dyn Protocol>> {
        Ok(Box::new((self.creator)()))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::built_in::{modbus::ModbusMode, LineProtocol, ModbusProtocol};

    #[tokio::test]
    async fn test_registry_registration() {
        let mut registry = ProtocolRegistry::new();

        // Register line protocol using Arc
        let factory = Arc::new(SimpleProtocolFactory::new(
            "line".to_string(),
            "Line-based protocol".to_string(),
            LineProtocol::new,
        ));
        registry.register(factory).await;

        // List protocols
        let protocols = registry.list_protocols().await;
        assert_eq!(protocols.len(), 1);
        assert_eq!(protocols[0].name, "line");
        assert_eq!(protocols[0].description, "Line-based protocol");
    }

    #[tokio::test]
    async fn test_get_protocol() {
        let mut registry = ProtocolRegistry::new();

        let factory = Arc::new(SimpleProtocolFactory::new(
            "modbus_rtu".to_string(),
            "Modbus RTU protocol".to_string(),
            || ModbusProtocol::new(ModbusMode::Rtu),
        ));
        registry.register(factory).await;

        // Get protocol
        let protocol = registry.get_protocol("modbus_rtu").await;
        assert!(protocol.is_ok());
        assert_eq!(protocol.unwrap().name(), "modbus_rtu");
    }

    #[tokio::test]
    async fn test_protocol_not_found() {
        let registry = ProtocolRegistry::new();
        let result = registry.get_protocol("nonexistent").await;
        assert!(result.is_err());
    }
}
