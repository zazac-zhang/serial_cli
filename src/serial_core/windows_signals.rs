//! Windows-specific signal control implementation
//!
//! This module provides real hardware signal control for Windows platforms
//! using the Windows API EscapeCommFunction.

#[cfg(windows)]
use crate::error::{Result, SerialError, SerialPortError};
#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

#[cfg(windows)]
use windows::Win32::Foundation::HANDLE;
#[cfg(windows)]
use windows::Win32::System::Comm::EscapeCommFunction;
#[cfg(windows)]
use windows::Win32::System::Comm::{CLRDTR, CLRRTS, SETDTR, SETRTS};

/// Windows signal control helper
#[cfg(windows)]
pub struct WindowsSignalControl {
    #[allow(dead_code)]
    port_name: String,
}

#[cfg(windows)]
impl WindowsSignalControl {
    /// Create a new Windows signal control helper
    pub fn new(port_name: String) -> Self {
        Self { port_name }
    }

    /// Open the COM port for signal control
    ///
    /// # Safety
    ///
    /// This function opens a file handle to the COM port for signal control.
    /// The handle should be closed when done.
    #[allow(dead_code)]
    pub fn open_port_handle(&self) -> Result<File> {
        File::open(&self.port_name).map_err(SerialError::Io)
    }

    /// Set DTR signal on a port
    ///
    /// # Arguments
    ///
    /// * `handle` - Raw Windows HANDLE to the COM port
    /// * `enable` - true to set DTR, false to clear DTR
    ///
    /// # Safety
    ///
    /// This function calls Windows API EscapeCommFunction which directly
    /// manipulates hardware. The handle must be valid and refer to a COM port.
    #[allow(dead_code)]
    pub unsafe fn set_dtr_handle(handle: HANDLE, enable: bool) -> Result<()> {
        let func = if enable { SETDTR } else { CLRDTR };

        let result = EscapeCommFunction(handle, func);
        if !result.as_bool() {
            let error_code = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            return Err(SerialError::Serial(SerialPortError::IoError(format!(
                "Failed to set DTR on Windows. Error code: {}",
                error_code
            ))));
        }

        tracing::debug!("DTR set to {} on Windows handle {:?}", enable, handle);
        Ok(())
    }

    /// Set RTS signal on a port
    ///
    /// # Arguments
    ///
    /// * `handle` - Raw Windows HANDLE to the COM port
    /// * `enable` - true to set RTS, false to clear RTS
    ///
    /// # Safety
    ///
    /// This function calls Windows API EscapeCommFunction which directly
    /// manipulates hardware. The handle must be valid and refer to a COM port.
    #[allow(dead_code)]
    pub unsafe fn set_rts_handle(handle: HANDLE, enable: bool) -> Result<()> {
        let func = if enable { SETRTS } else { CLRRTS };

        let result = EscapeCommFunction(handle, func);
        if !result.as_bool() {
            let error_code = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            return Err(SerialError::Serial(SerialPortError::IoError(format!(
                "Failed to set RTS on Windows. Error code: {}",
                error_code
            ))));
        }

        tracing::debug!("RTS set to {} on Windows handle {:?}", enable, handle);
        Ok(())
    }

    /// Set DTR signal by port name (convenience method)
    ///
    /// This method opens the port, sets the signal, and closes the port.
    /// Note that this is less efficient than keeping the port open.
    #[allow(dead_code)]
    pub fn set_dtr_by_name(&self, enable: bool) -> Result<()> {
        let file = self.open_port_handle()?;
        let handle = file.as_raw_handle();

        unsafe { Self::set_dtr_handle(handle, enable) }
    }

    /// Set RTS signal by port name (convenience method)
    ///
    /// This method opens the port, sets the signal, and closes the port.
    /// Note that this is less efficient than keeping the port open.
    #[allow(dead_code)]
    pub fn set_rts_by_name(&self, enable: bool) -> Result<()> {
        let file = self.open_port_handle()?;
        let handle = file.as_raw_handle();

        unsafe { Self::set_rts_handle(handle, enable) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_windows_signal_control_creation() {
        let control = WindowsSignalControl::new("COM1".to_string());
        assert_eq!(control.port_name, "COM1");
    }

    #[test]
    #[cfg(windows)]
    fn test_set_dtr_by_name() {
        let control = WindowsSignalControl::new("COM1".to_string());
        // This will fail if COM1 doesn't exist, but the call should not panic
        let _ = control.set_dtr_by_name(true);
        // API is callable and returns Result — we verify it doesn't crash
    }

    #[test]
    #[cfg(windows)]
    fn test_set_rts_by_name() {
        let control = WindowsSignalControl::new("COM1".to_string());
        // This will fail if COM1 doesn't exist, but the call should not panic
        let _ = control.set_rts_by_name(true);
        // API is callable and returns Result — we verify it doesn't crash
    }
}
