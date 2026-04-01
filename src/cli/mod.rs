//! CLI interface module
//!
//! This module provides command-line interface functionality.

pub mod interactive;
pub mod batch;
pub mod commands;
pub mod json;

pub use json::JsonFormatter;
