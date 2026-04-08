//! CLI interface module
//!
//! This module provides command-line interface functionality.

pub mod batch;
pub mod commands;
pub mod interactive;
pub mod json;

pub use json::JsonFormatter;
