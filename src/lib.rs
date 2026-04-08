//! Serial CLI - A universal serial port tool optimized for AI interaction
//!
//! This library provides the core functionality for the serial-cli tool,
//! including serial port management, protocol handling, and Lua scripting.

pub mod config;
pub mod error;

pub mod cli;
pub mod lua;
pub mod protocol;
pub mod serial_core;
pub mod task;

// Re-exports for convenience
pub use error::{Result, SerialError};
