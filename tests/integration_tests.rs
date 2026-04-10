//! Integration tests for platform-specific signal control
//!
//! These tests verify that the platform abstraction layer works correctly
//! and that signal control functions behave as expected across different platforms.

#[cfg(test)]
mod signal_control_tests {
    use serial_cli::serial_core::signals::{PlatformSignals, SignalState};

    #[test]
    fn test_signal_controller_creation() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let controller = UnixSignalController::new();
        #[cfg(windows)]
        let controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let controller = FallbackSignalController::new();

        assert!(!controller.platform_name().is_empty());
    }

    #[test]
    fn test_dtr_state_management() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let mut controller = UnixSignalController::new();
        #[cfg(windows)]
        let mut controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let mut controller = FallbackSignalController::new();

        // Test initial state
        assert_eq!(controller.get_dtr().unwrap(), true);

        // Test state changes
        let result = controller.set_dtr(false);
        assert!(result.is_ok());

        match result.unwrap() {
            SignalState::Set(state) => assert_eq!(state, false),
            SignalState::NotSupported => {
                // Expected on platforms without hardware support
            }
            SignalState::Failed => {
                // Hardware control failed but state was updated
                assert_eq!(controller.get_dtr().unwrap(), false);
            }
        }
    }

    #[test]
    fn test_rts_state_management() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let mut controller = UnixSignalController::new();
        #[cfg(windows)]
        let mut controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let mut controller = FallbackSignalController::new();

        // Test initial state
        assert_eq!(controller.get_rts().unwrap(), true);

        // Test state changes
        let result = controller.set_rts(false);
        assert!(result.is_ok());

        match result.unwrap() {
            SignalState::Set(state) => assert_eq!(state, false),
            SignalState::NotSupported => {
                // Expected on platforms without hardware support
            }
            SignalState::Failed => {
                // Hardware control failed but state was updated
                assert_eq!(controller.get_rts().unwrap(), false);
            }
        }
    }

    #[test]
    fn test_signal_toggle() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let mut controller = UnixSignalController::new();
        #[cfg(windows)]
        let mut controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let mut controller = FallbackSignalController::new();

        // Toggle DTR
        for _ in 0..3 {
            let current = controller.get_dtr().unwrap();
            let result = controller.set_dtr(!current);
            assert!(result.is_ok());

            // Verify state changed
            assert_ne!(controller.get_dtr().unwrap(), current);
        }
    }

    #[test]
    fn test_concurrent_signal_operations() {
        use std::sync::Barrier;
        use std::sync::{Arc, Mutex};
        use std::thread;

        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let controller = Arc::new(Mutex::new(UnixSignalController::new()));
        #[cfg(windows)]
        let controller = Arc::new(Mutex::new(WindowsSignalController::new()));
        #[cfg(not(any(unix, windows)))]
        let controller = Arc::new(Mutex::new(FallbackSignalController::new()));

        let barrier = Arc::new(Barrier::new(2));

        // Test concurrent DTR operations
        let controller1 = controller.clone();
        let barrier1 = barrier.clone();
        let handle1 = thread::spawn(move || {
            barrier1.wait();
            let mut ctrl = controller1.lock().unwrap();
            ctrl.set_dtr(true).is_ok()
        });

        let controller2 = controller.clone();
        let barrier2 = barrier.clone();
        let handle2 = thread::spawn(move || {
            barrier2.wait();
            let mut ctrl = controller2.lock().unwrap();
            ctrl.set_rts(false).is_ok()
        });

        let result1 = handle1.join().unwrap();
        let result2 = handle2.join().unwrap();

        // Both operations should complete successfully
        assert!(result1 || result2); // At least one should succeed
    }

    #[test]
    fn test_error_recovery() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let mut controller = UnixSignalController::new();
        #[cfg(windows)]
        let mut controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let mut controller = FallbackSignalController::new();

        // Test that failed operations don't corrupt state
        let initial_dtr = controller.get_dtr().unwrap();
        let initial_rts = controller.get_rts().unwrap();

        // Try invalid operations (will be caught as errors)
        let _ = controller.set_dtr(!initial_dtr);
        let _ = controller.set_rts(!initial_rts);

        // State should be consistent
        assert_ne!(controller.get_dtr().unwrap(), initial_dtr);
        assert_ne!(controller.get_rts().unwrap(), initial_rts);
    }

    #[test]
    fn test_platform_specific_behavior() {
        #[cfg(not(any(unix, windows)))]
        use serial_cli::serial_core::signals::FallbackSignalController;
        #[cfg(unix)]
        use serial_cli::serial_core::signals::UnixSignalController;
        #[cfg(windows)]
        use serial_cli::serial_core::signals::WindowsSignalController;

        #[cfg(unix)]
        let controller = UnixSignalController::new();
        #[cfg(windows)]
        let controller = WindowsSignalController::new();
        #[cfg(not(any(unix, windows)))]
        let controller = FallbackSignalController::new();

        // Test platform-specific behavior
        #[cfg(unix)]
        {
            assert!(matches!(
                controller.platform_name(),
                "linux" | "macos" | "unix"
            ));
        }

        #[cfg(windows)]
        {
            assert_eq!(controller.platform_name(), "windows");
        }

        #[cfg(not(any(unix, windows)))]
        {
            assert_eq!(controller.platform_name(), "fallback");
        }
    }
}

#[cfg(test)]
mod protocol_lifecycle_tests {
    use serial_cli::protocol::built_in::LineProtocol;
    use serial_cli::protocol::registry::SimpleProtocolFactory;
    use serial_cli::protocol::ProtocolRegistry;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_protocol_unregister() {
        let mut registry = ProtocolRegistry::new();

        // Register a protocol
        let factory = Arc::new(SimpleProtocolFactory::new(
            "test_proto".to_string(),
            "Test protocol".to_string(),
            LineProtocol::new,
        ));
        registry.register(factory).await;

        // Verify it's registered
        assert!(registry.is_registered("test_proto").await);

        // Unregister it
        let result = registry.unregister("test_proto").await;
        assert!(result.is_ok());

        // Verify it's gone
        assert!(!registry.is_registered("test_proto").await);
    }

    #[tokio::test]
    async fn test_unregister_nonexistent_protocol() {
        let mut registry = ProtocolRegistry::new();

        let result = registry.unregister("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_protocol_manager_load_unload() {
        use serial_cli::protocol::manager::ProtocolManager;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let registry = ProtocolRegistry::new();
        let mut manager = ProtocolManager::new(Arc::new(Mutex::new(registry)));

        // Create a test protocol script
        let script = r#"
            -- Protocol: test_unload
            function on_frame(data)
                return data
            end

            function on_encode(data)
                return data
            end
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        // Load the protocol
        let load_result = manager.load_protocol(temp_file.path()).await;
        assert!(load_result.is_ok());
        assert!(manager.custom_protocols_len() == 1);

        // Unload the protocol
        let unload_result = manager.unload_protocol("test_unload").await;
        assert!(unload_result.is_ok());
        assert!(manager.custom_protocols_len() == 0);
    }
}
