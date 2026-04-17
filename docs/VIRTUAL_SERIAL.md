# Virtual Serial Port Pair Implementation

## Overview

This document describes the design and implementation of virtual serial port pairs for Serial CLI, enabling creation of pseudo-serial ports for testing, monitoring, and debugging purposes.

## Use Cases

### 1. Serial Communication Testing
- Test serial protocols without hardware
- Automated testing in CI/CD pipelines
- Development and debugging of serial applications

### 2. Traffic Monitoring
- Monitor bidirectional serial communication
- Debug protocol implementations
- Analyze data flow between applications

### 3. Integration Testing
- Test serial port handling logic
- Verify protocol parsers
- Simulate device behavior

## Architecture

### Design Principles

1. **Platform-Specific Implementation**: Use native APIs for each platform
2. **Zero External Dependencies**: Leverage existing Rust ecosystem
3. **Seamless Integration**: Work with existing `PortManager` and `SerialSniffer`
4. **Transparent Monitoring**: Built-in traffic capture capabilities

### Component Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Interface                         │
│  serial-cli virtual create/list/stop/stats              │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│              VirtualPortManager                         │
│  - Manages virtual port lifecycle                        │
│  - Integrates with PortManager                           │
│  - Coordinates monitoring                               │
└────────────────────────┬────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
┌───────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
│   PTY        │ │  NamedPipe  │ │   Socat     │
│  (Unix/macOS)│ │  (Windows)  │ │  (Cross-plat)│
└──────────────┘ └─────────────┘ └─────────────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
        ┌────────────────▼────────────────┐
        │      SerialSniffer Integration  │
        │  - Capture TX/RX traffic        │
        │  - Real-time monitoring         │
        │  - Export to file               │
        └─────────────────────────────────┘
```

## Platform Implementations

### Unix/Linux (PTY)

**Mechanism**: POSIX PTY (Pseudo-Terminal)

**API Calls**:
- `posix_openpt()` - Open PTY master
- `grantpt()` - Grant slave PTY
- `unlockpt()` - Unlock slave PTY
- `ptsname()` - Get slave PTY name

**Example Port Names**:
- Master: `/dev/pts/0`
- Slave: `/dev/pts/1`

**Advantages**:
- ✅ Native kernel support
- ✅ Low latency
- ✅ No external dependencies
- ✅ Full serial port features

**Limitations**:
- ⚠️ Requires `libc` wrapper
- ⚠️ Platform-specific constants

### macOS (PTY)

**Mechanism**: Similar to Unix but with different constants

**Differences from Linux**:
- Uses `libc` with BSD-specific constants
- May require additional permissions
- PTY behavior slightly different

**Implementation Status**: ⚠️ Needs testing and potential workarounds

### Windows (Named Pipe)

**Mechanism**: Windows Named Pipes

**API**:
- `CreateNamedPipe()` - Create named pipe server
- `CreateFile()` - Connect to named pipe
- Pipe names: `\\.\pipe\serial_virt_X`

**Advantages**:
- ✅ Native Windows API
- ✅ Good performance
- ✅ No driver installation

**Limitations**:
- ⚠️ Not real COM ports (some apps may not accept)
- ⚠️ Different naming scheme

### Cross-Platform Alternative (socat)

**Mechanism**: External `socat` process

**Command**:
```bash
socat -d -d pty,raw,echo=0 pty,raw,echo=0
```

**Advantages**:
- ✅ Works on all platforms
- ✅ Quick to implement
- ✅ Well-tested tool

**Limitations**:
- ⚠️ Requires external binary
- ⚠️ Process management overhead
- ⚠️ Dependency on system package manager

## Implementation Plan

### Phase 1: Unix PTY Implementation ✅ (Priority)

**Tasks**:
1. Implement `VirtualSerialPair` using `libc`
2. Handle PTY lifecycle (creation, cleanup)
3. Add error handling for permissions
4. Write unit tests

**Files**:
- `src/serial_core/virtual.rs` - Core implementation
- Update `src/serial_core/mod.rs` - Export module
- Update `src/main.rs` - Add CLI commands

**Estimated Time**: 1-2 days

### Phase 2: Integration with Existing Systems

**Tasks**:
1. Integrate with `PortManager`
2. Add `SerialSniffer` monitoring
3. Implement statistics collection
4. Add cleanup on exit

**Estimated Time**: 1 day

### Phase 3: CLI Interface

**Tasks**:
1. Add `virtual` subcommand
2. Implement `create/list/stop/stats` commands
3. Add monitoring options
4. Add output formatting

**Estimated Time**: 0.5 day

### Phase 4: Windows Support

**Tasks**:
1. Implement Named Pipe backend
2. Test on Windows
3. Document limitations

**Estimated Time**: 2-3 days

## API Design

### Core API

```rust
/// Virtual serial port pair
pub struct VirtualSerialPair {
    /// Port A name (e.g., /dev/pts/0)
    pub port_a: String,

    /// Port B name (e.g., /dev/pts/1)
    pub port_b: String,

    /// Unique identifier
    pub id: String,

    /// Backend type
    backend: VirtualBackend,

    /// Active sniffer session
    sniffer: Option<SnifferSession>,

    /// Running state
    running: Arc<AtomicBool>,
}

impl VirtualSerialPair {
    /// Create a new virtual serial port pair
    pub async fn create(config: VirtualConfig) -> Result<Self>;

    /// Start monitoring traffic
    pub async fn start_monitoring(&mut self) -> Result<()>;

    /// Stop monitoring and cleanup
    pub async fn stop(self) -> Result<()>;

    /// Get statistics
    pub async fn stats(&self) -> Result<VirtualStats>;
}
```

### CLI Interface

```bash
# Create virtual port pair with monitoring
serial-cli virtual create --backend pty --monitor --output traffic.log

# List active virtual port pairs
serial-cli virtual list

# Stop a specific virtual port pair
serial-cli virtual stop <id>

# Show statistics
serial-cli virtual stats <id>
```

### Configuration

```toml
[virtual]
# Default backend type
backend = "pty"

# Enable monitoring by default
monitor = true

# Monitoring output format
monitor_format = "hex"  # hex or raw

# Auto-cleanup on exit
auto_cleanup = true
```

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `PermissionDenied` | Insufficient permissions | Run with appropriate privileges |
| `BackendUnavailable` | Backend not supported on platform | Use alternative backend |
| `PortCreationFailed` | System resource exhaustion | Close unused ports |
| `MonitorError` | Sniffer initialization failed | Check disk space for output |

### Platform-Specific Errors

**Unix**:
- PTY creation failure (resource limits)
- Permission denied (devpts configuration)

**Windows**:
- Named pipe creation failure (permissions)
- Pipe name conflicts

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_virtual_pair_creation() {
    let pair = VirtualSerialPair::create(VirtualConfig::default()).unwrap();
    assert!(!pair.port_a.is_empty());
    assert!(!pair.port_b.is_empty());
}

#[tokio::test]
async fn test_bidirectional_communication() {
    let pair = VirtualSerialPair::create(VirtualConfig::default()).await.unwrap();

    // Open both ports
    let port_a = PortManager::open_port(&pair.port_a, SerialConfig::default()).await?;
    let port_b = PortManager::open_port(&pair.port_b, SerialConfig::default()).await?;

    // Test communication
    // ...
}
```

### Integration Tests

1. **Protocol Testing**: Test with built-in protocols (Modbus, AT Command)
2. **Monitoring**: Verify sniffer captures all traffic
3. **Cleanup**: Ensure proper resource cleanup
4. **Stress Testing**: Multiple concurrent virtual pairs

## Performance Considerations

### Benchmarks

**Latency**:
- PTY: < 1ms
- Named Pipe: < 2ms
- Socat: 2-5ms (process overhead)

**Throughput**:
- PTY: > 100 MB/s
- Named Pipe: > 80 MB/s
- Socat: ~50 MB/s

### Optimization Strategies

1. **Zero-Copy**: Use shared buffers where possible
2. **Async I/O**: Leverage tokio for non-blocking operations
3. **Batch Processing**: Group small packets
4. **Memory Pooling**: Reuse buffers for monitoring

## Security Considerations

### Risks

1. **Data Leakage**: Captured traffic may contain sensitive data
2. **Resource Exhaustion**: Unbounded port creation
3. **Privilege Escalation**: PTY creation on Unix

### Mitigations

1. **Permission Checks**: Verify user has appropriate privileges
2. **Rate Limiting**: Limit concurrent virtual pairs
3. **Secure Cleanup**: Ensure sensitive data is cleared
4. **Access Control**: Restrict who can create virtual ports

## Future Enhancements

### Planned Features

1. **Protocol Simulation**: Built-in device simulators
2. **Traffic Replay**: Record and replay serial traffic
3. **Virtual Modem**: AT command response simulation
4. **Network Bridging**: Bridge virtual serial to network sockets

### Potential Improvements

1. **GUI Integration**: Add virtual port creation to Tauri GUI
2. **Lua Automation**: Script virtual port scenarios
3. **Configuration Profiles**: Save/load virtual port configurations
4. **Hot-Swapping**: Dynamically add/remove virtual ports

## Troubleshooting

### Common Issues

**Issue**: "Failed to create PTY"
- **Solution**: Check system PTY limits (`sysctl kernel.pid_max`)
- **Solution**: Verify permissions on `/dev/ptmx`

**Issue**: "Ports not visible in listing"
- **Solution**: Ensure cleanup completed successfully
- **Solution**: Check for zombie processes

**Issue**: "Monitoring not capturing data"
- **Solution**: Verify sniffer is started before communication
- **Solution**: Check disk space for output file

## References

### Documentation
- [POSIX PTY](https://man7.org/linux/man-pages/man7/pty.7.html)
- [Windows Named Pipes](https://docs.microsoft.com/en-us/windows/win32/ipc/named-pipes)
- [socat Manual](http://www.dest-unreach.org/socat/)

### Similar Projects
- [com0com](http://com0com.sourceforge.net/) - Windows virtual serial port driver
- [tty0tty](https://github.com/freemed/tty0tty) - Linux virtual serial ports
- [socat](http://www.dest-unreach.org/socat/) - Multi-purpose relay

## Changelog

### 2026-04-17
- Initial design document created
- Unix PTY implementation planned
- Integration architecture defined
