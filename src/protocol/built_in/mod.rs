//! Built-in protocols

pub mod at_command;
pub mod line;
pub mod modbus;

pub use at_command::AtCommandProtocol;
pub use line::LineProtocol;
pub use modbus::ModbusProtocol;

/// Names of built-in protocols (cannot be used for custom scripts).
pub const BUILTIN_PROTOCOL_NAMES: &[&str] = &["modbus_rtu", "modbus_ascii", "at_command", "line"];

/// Check if a protocol name is a built-in.
pub fn is_builtin_protocol(name: &str) -> bool {
    BUILTIN_PROTOCOL_NAMES.contains(&name)
}
