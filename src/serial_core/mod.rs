//! Serial core module
//!
//! This module provides serial port management and I/O operations.

pub mod io_loop;
pub mod port;
pub mod signals;
pub mod sniffer;

#[cfg(windows)]
pub mod windows_signals;

pub use io_loop::IoLoop;
pub use port::{FlowControl, Parity, PortManager, SerialConfig, SerialPortHandle};
pub use signals::{create_signal_controller, PlatformSignals, SignalState};
pub use sniffer::{CapturedPacket, PacketDirection, SerialSniffer, SnifferConfig, SnifferSession};

#[cfg(windows)]
pub use windows_signals::WindowsSignalControl;
