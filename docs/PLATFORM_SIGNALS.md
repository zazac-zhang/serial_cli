# Platform Signal Control Architecture

## Overview

This document describes the architecture and implementation of platform-specific signal control (DTR/RTS) in the serial-cli application.

## Design Principles

### 1. Platform Abstraction
All platform-specific code is abstracted behind the `PlatformSignals` trait, providing a unified interface across different operating systems.

### 2. Safety First
Unsafe code is minimized and properly documented with:
- Clear preconditions
- Validation steps
- Error recovery mechanisms
- Detailed safety comments

### 3. Graceful Degradation
The system follows a "warn but don't fail" approach:
- Hardware control failures are logged but don't crash the application
- State is maintained in memory even if hardware control fails
- Clear distinction between supported and unsupported platforms

## Architecture

```
┌─────────────────────────────────────────────┐
│         SerialPortHandle                   │
│  ┌───────────────────────────────────────┐  │
│  │     signal_controller: Arc<dyn      │  │
│  │     PlatformSignals>                  │  │
│  └───────────────────────────────────────┘  │
│              ▲                               │
│              │                              │
│  ┌─────────────┴──────────────────────┐   │
│  │   Platform-specific implementations   │   │
│  │  ┌─────────────────────────────┐    │   │
│  │  │ UnixSignalController        │    │   │
│  │  │ - ioctl TIOCMGET/TIOCMSET  │    │   │
│  │  │ - fd validation            │    │   │
│  │  └─────────────────────────────┘    │   │
│  │  ┌─────────────────────────────┐    │   │
│  │  │ WindowsSignalController     │    │   │
│  │  │ - EscapeCommFunction        │    │   │
│  │  │ - handle validation        │    │   │
│  │  └─────────────────────────────┘    │   │
│  │  ┌─────────────────────────────┐    │   │
│  │  │ FallbackSignalController    │    │   │
│  │  │ - memory-only state         │    │   │
│  │  └─────────────────────────────┘    │   │
│  └───────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

## Signal States

The system uses three possible states after signal operations:

1. **Set(bool)**: Signal successfully set to the requested value
2. **NotSupported**: Platform doesn't support this signal
3. **Failed**: Operation failed but state updated in memory

## Error Handling Strategy

### Consistent Approach
All signal control follows this pattern:

```rust
match signal_controller.set_dtr(enable) {
    Ok(SignalState::Set(actual)) => {
        // Success - log the actual state
    }
    Ok(SignalState::NotSupported) => {
        // Expected on some platforms - log info
    }
    Ok(SignalState::Failed) => {
        // Hardware failed but app continues - log warning
    }
    Err(e) => {
        // Critical error - log error and return
    }
}
```

### Unix Implementation Details

#### Modem Control Bits
```c
// TIOCM bit definitions (from <sys/ttycom.h>)
#define TIOCM_LE	0x001	// DTR (Data Terminal Ready)
#define TIOCM_DTR	0x002	// RTS (Request to Send)
#define TIOCM_ST	0x004	// Secondary Transmit Data
#define TIOCM_SR	0x008	// Secondary Receive Data
#define TIOCM_CTS	0x020	// Clear to Send
#define TIOCM_CAR	0x040	// Data Carrier Detect
#define TIOCM_RNG	0x080	// Ring Indicator
#define TIOCM_DSR	0x100	// Data Set Ready
```

#### ioctl Operations
```c
// Get current modem status
int ioctl(int fd, TIOCMGET, int *status);

// Set modem status
int ioctl(int fd, TIOCMSET, int *status);
```

#### Safety Validation
1. **File Descriptor Validation**: Check fd >= 0
2. **TTY Verification**: Use tcgetattr to ensure fd is a terminal
3. **Error Recovery**: Proper error messages and logging

## Testing Strategy

### Unit Tests
- Controller creation and state management
- Signal toggle operations
- Error conditions and recovery

### Integration Tests
- Protocol load/unload lifecycle
- Concurrent signal operations
- Cross-platform behavior verification

### Platform-Specific Tests
- Unix: ioctl operations with real TTY devices
- Windows: EscapeCommFunction with serial ports
- Fallback: Memory-only state management

## Future Improvements

1. **Hardware Testing**: Add tests with actual serial hardware
2. **Performance Monitoring**: Track signal control operation timing
3. **Enhanced Error Recovery**: Retry logic for transient failures
4. **Signal Monitoring**: Add functions to read actual signal state from hardware
5. **Event Notification**: Signal state change callbacks