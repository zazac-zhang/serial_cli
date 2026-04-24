//! File watcher for protocol hot-reload
//!
//! Monitors Lua protocol files for changes and triggers reloads.

use crate::error::{ProtocolError, Result, SerialError};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

/// File watcher for protocol scripts
pub struct ProtocolWatcher {
    _watcher: RecommendedWatcher,
    reload_tx: mpsc::UnboundedSender<PathBuf>,
    reload_rx: Option<mpsc::UnboundedReceiver<PathBuf>>,
}

impl ProtocolWatcher {
    /// Create a new file watcher
    pub fn new() -> Result<Self> {
        let (reload_tx, reload_rx) = mpsc::unbounded_channel::<PathBuf>();

        // Clone tx for struct storage (UnboundedSender is cloneable)
        let struct_tx = reload_tx.clone();

        // Create watcher
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                match event.kind {
                    notify::EventKind::Modify(_)
                    | notify::EventKind::Create(_)
                    | notify::EventKind::Remove(_) => {
                        for path in event.paths {
                            // Only process .lua files
                            if path.extension().and_then(|s: &std::ffi::OsStr| s.to_str())
                                == Some("lua")
                            {
                                let _ = reload_tx.send(path);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
        .map_err(|e| {
            SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Failed to create watcher: {}",
                e
            )))
        })?;

        Ok(Self {
            _watcher: watcher,
            reload_tx: struct_tx,
            reload_rx: Some(reload_rx),
        })
    }

    /// Watch a file for changes
    pub fn watch(&mut self, path: &Path) -> Result<()> {
        // Watch the parent directory
        let parent = path.parent().unwrap_or(path);

        self._watcher
            .watch(parent, RecursiveMode::NonRecursive)
            .map_err(|e| {
                SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                    "Failed to watch path: {}",
                    e
                )))
            })?;

        Ok(())
    }

    /// Get reload event receiver.
    /// Returns `None` if already taken.
    pub fn reload_events(&mut self) -> Option<mpsc::UnboundedReceiver<PathBuf>> {
        self.reload_rx.take()
    }

    /// Get a clone of the reload sender (for testing or forwarding).
    pub fn sender(&self) -> mpsc::UnboundedSender<PathBuf> {
        self.reload_tx.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watcher_creation() {
        let watcher = ProtocolWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_watch_valid_path() {
        let mut watcher = ProtocolWatcher::new().unwrap();
        let result = watcher.watch(Path::new("tests/fixtures/protocols/test_valid.lua"));
        // Should succeed since the file exists
        assert!(result.is_ok());
    }

    #[test]
    fn test_watch_nonexistent_path() {
        let mut watcher = ProtocolWatcher::new().unwrap();
        // Watching a nonexistent directory should fail
        let result = watcher.watch(Path::new("/nonexistent/path/file.lua"));
        assert!(result.is_err());
    }
}
