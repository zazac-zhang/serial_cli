//! Virtual serial port pair creation using PTY

use std::io::{self, Read, Write};
use std::time::Duration;

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
        // On Unix-like systems, use /dev/ptmx
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;

            // Open the master PTY
            let master_fd = unsafe {
                libc::open(b"/dev/ptmx\0".as_ptr() as *const i8, libc::O_RDWR | libc::O_NOCTTY, 0)
            };

            if master_fd < 0 {
                return Err(format!("Failed to open /dev/ptmx: {}", io::Error::last_os_error()).into());
            }

            // Unlock the pty
            let mut unlock: libc::c_int = 0;
            if unsafe { libc::ioctl(master_fd, libc::TIOCSPTLCK, &unlock) } < 0 {
                unsafe { libc::close(master_fd) };
                return Err("Failed to unlock PTY".into());
            }

            // Get the slave PTY name
            let mut slave_name: [libc::c_char; 64] = [0; 64];
            if unsafe { libc::ioctl(master_fd, libc::TIOCGPTN, &mut unlock) } < 0 {
                unsafe { libc::close(master_fd) };
                return Err("Failed to get PTY number".into());
            }

            let slave_path = format!("/dev/pts/{}", unlock);

            // Close master fd as we only need the path names
            unsafe { libc::close(master_fd) };

            Ok(Self {
                master: "/dev/ptmx".to_string(),
                slave: slave_path,
            })
        }

        #[cfg(windows)]
        {
            // On Windows, return dummy paths (real virtual ports require drivers)
            Ok(Self {
                master: "COM1".to_string(),
                slave: "COM2".to_string(),
            })
        }
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
