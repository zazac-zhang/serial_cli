//! Protocol manager tests

use serial_cli::protocol::{ProtocolManager, ProtocolRegistry};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_manager_creation() {
    let registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
    let manager = ProtocolManager::new(registry);
    assert_eq!(manager.custom_protocols_len(), 0);
}
