//! Serial core module
//!
//! This module provides serial port management and I/O operations.

pub mod port;
pub mod io_loop;

pub use port::{PortManager, SerialConfig, SerialPortHandle, Parity};
pub use io_loop::IoLoop;
