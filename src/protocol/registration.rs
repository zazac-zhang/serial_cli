//! Built-in protocol registration
//!
//! Registers all built-in protocols to the protocol registry.

use crate::error::Result;
use crate::protocol::ProtocolFactory;
use crate::protocol::built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol};
use crate::protocol::built_in::modbus::ModbusMode;
use crate::protocol::ProtocolRegistry;
use std::sync::Arc;
use tokio::sync::Mutex;

// Modbus RTU factory
struct ModbusRtuFactory;
impl ProtocolFactory for ModbusRtuFactory {
    fn create(&self) -> Result<Box<dyn crate::protocol::Protocol>> {
        Ok(Box::new(ModbusProtocol::new(ModbusMode::Rtu)))
    }
    fn name(&self) -> &str {
        "modbus_rtu"
    }
    fn description(&self) -> &str {
        "Modbus RTU protocol (Binary industrial communication)"
    }
}

// Modbus ASCII factory
struct ModbusAsciiFactory;
impl ProtocolFactory for ModbusAsciiFactory {
    fn create(&self) -> Result<Box<dyn crate::protocol::Protocol>> {
        Ok(Box::new(ModbusProtocol::new(ModbusMode::Ascii)))
    }
    fn name(&self) -> &str {
        "modbus_ascii"
    }
    fn description(&self) -> &str {
        "Modbus ASCII protocol (Text-based industrial communication)"
    }
}

// AT Command factory
struct AtCommandFactory;
impl ProtocolFactory for AtCommandFactory {
    fn create(&self) -> Result<Box<dyn crate::protocol::Protocol>> {
        Ok(Box::new(AtCommandProtocol::new()))
    }
    fn name(&self) -> &str {
        "at_command"
    }
    fn description(&self) -> &str {
        "AT Command protocol (Modem control commands)"
    }
}

// Line protocol factory
struct LineProtocolFactory;
impl ProtocolFactory for LineProtocolFactory {
    fn create(&self) -> Result<Box<dyn crate::protocol::Protocol>> {
        Ok(Box::new(LineProtocol::new()))
    }
    fn name(&self) -> &str {
        "line"
    }
    fn description(&self) -> &str {
        "Line-based protocol (Simple text line communication)"
    }
}

/// Register all built-in protocols to the registry
pub async fn register_all_built_in(registry: Arc<Mutex<ProtocolRegistry>>) -> Result<()> {
    let factories: Vec<Arc<dyn ProtocolFactory>> = vec![
        Arc::new(ModbusRtuFactory),
        Arc::new(ModbusAsciiFactory),
        Arc::new(AtCommandFactory),
        Arc::new(LineProtocolFactory),
    ];

    let mut reg = registry.lock().await;
    for factory in &factories {
        reg.register(Arc::clone(factory)).await;
    }

    tracing::debug!("Registered {} built-in protocols", factories.len());
    Ok(())
}
