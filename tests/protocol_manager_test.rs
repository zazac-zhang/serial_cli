//! Protocol manager tests

use serial_cli::protocol::{ProtocolManager, ProtocolRegistry, ProtocolValidator};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_manager_creation() {
    let registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
    let manager = ProtocolManager::new(registry);
    assert_eq!(manager.custom_protocols_len(), 0);
}

#[tokio::test]
async fn test_load_valid_protocol() {
    let registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
    let mut manager = ProtocolManager::new(registry);

    let path = std::path::PathBuf::from("tests/fixtures/protocols/test_valid.lua");
    let result = manager.load_protocol(&path).await;

    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.name, "test_valid");
}

#[tokio::test]
async fn test_load_invalid_protocol() {
    let registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
    let mut manager = ProtocolManager::new(registry);

    let path = std::path::PathBuf::from("tests/fixtures/protocols/test_syntax_error.lua");
    let result = manager.load_protocol(&path).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_validate_protocol() {
    let valid_path = std::path::PathBuf::from("tests/fixtures/protocols/test_valid.lua");
    assert!(ProtocolValidator::validate_script(&valid_path).is_ok());

    let error_path = std::path::PathBuf::from("tests/fixtures/protocols/test_syntax_error.lua");
    assert!(ProtocolValidator::validate_script(&error_path).is_err());

    let missing_func_path = std::path::PathBuf::from("tests/fixtures/protocols/test_missing_func.lua");
    assert!(ProtocolValidator::validate_script(&missing_func_path).is_err());
}

#[tokio::test]
async fn test_list_protocols() {
    let registry = Arc::new(Mutex::new(ProtocolRegistry::new()));
    let mut manager = ProtocolManager::new(registry);

    let path = std::path::PathBuf::from("tests/fixtures/protocols/test_valid.lua");
    manager.load_protocol(&path).await.unwrap();

    // Check that protocol is tracked in manager
    assert!(manager.get_custom_protocol("test_valid").is_some());
    assert_eq!(manager.custom_protocols_len(), 1);
}
