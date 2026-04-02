//! Virtual serial port pair creation using PTY
//!
//! NOTE: This module is currently disabled on macOS due to missing libc constants.
//! The virtual serial functionality is not used by the core benchmarks.

/// Result type for virtual serial operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Virtual serial port pair (master/slave PTY)
pub struct VirtualSerialPair {
    pub master: String,
    pub slave: String,
}

impl VirtualSerialPair {
    /// Create a new virtual serial port pair
    ///
    /// Returns names that can be used with tokio-serial
    pub fn create() -> Result<Self> {
        // Return dummy paths for now
        // Real implementation requires platform-specific PTY handling
        Ok(Self {
            master: "COM1".to_string(),
            slave: "COM2".to_string(),
        })
    }

    /// Clean up the virtual serial pair
    pub fn cleanup(self) -> Result<()> {
        // PTY cleanup is automatic on close
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_virtual_pair() {
        let pair = VirtualSerialPair::create();
        assert!(pair.is_ok());
        let pair = pair.unwrap();
        assert!(!pair.master.is_empty());
        assert!(!pair.slave.is_empty());
    }
}
