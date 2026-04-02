# Windows Platform Improvements TODO

This document tracks Windows-specific improvements needed for the serial-cli project.

## Priority Levels

- 🔴 **High**: Critical for Windows user experience
- 🟡 **Medium**: Important but not blocking
- 🟢 **Low**: Nice to have improvements

---

## 🔴 High Priority

### 1. Flow Control Support
**Status**: Not Implemented
**Impact**: Many Windows serial devices require hardware/software flow control

**Tasks**:
- [ ] Add `FlowControl` enum (None, Software, Hardware)
- [ ] Integrate with `SerialConfig`
- [ ] Update tokio-serial builder to apply flow control settings
- [ ] Add Lua API for flow control configuration
- [ ] Add tests for flow control modes

**Files to modify**:
- `src/serial_core/port.rs`
- `src/lua/bindings.rs`

---

### 2. DTR/RTS Signal Control
**Status**: Not Implemented
**Impact**: Many industrial devices require manual DTR/RTS control

**Tasks**:
- [ ] Add `dtr_enable` and `rts_enable` to `SerialConfig`
- [ ] Implement runtime DTR/RTS control methods
- [ ] Add Lua API for signal control
- [ ] Add documentation for device-specific requirements

**Files to modify**:
- `src/serial_core/port.rs`
- `src/lua/bindings.rs`

---

### 3. Enhanced Error Messages
**Status**: Partial
**Impact**: Users struggle to understand permission issues

**Tasks**:
- [ ] Add "Access is denied" to permission error detection
- [ ] Include helpful suggestions in error messages
- [ ] Add administrator privilege detection
- [ ] Provide port-in-use detection and suggestions
- [ ] Add multi-language error message support

**Files to modify**:
- `src/error.rs`
- `src/serial_core/port.rs`

---

## 🟡 Medium Priority

### 4. Enhanced Port Enumeration
**Status**: Basic
**Impact**: Difficult to identify devices in Windows Device Manager

**Tasks**:
- [ ] Add `friendly_name` field to `SerialPortInfo`
- [ ] Add `hardware_id` field for device identification
- [ ] Add `manufacturer` field
- [ ] Implement Windows registry querying for port details
- [ ] Add COM port number extraction from port name
- [ ] Add device class information

**Files to modify**:
- `src/serial_core/port.rs`
- `src/cli/commands.rs`

---

### 5. Windows-Specific Timeouts
**Status**: Basic
**Impact**: May cause performance issues on Windows

**Tasks**:
- [ ] Add separate read/write timeout configuration
- [ ] Add interval timeout support
- [ ] Implement Windows-specific timeout optimization
- [ ] Add timeout tuning recommendations in documentation

**Files to modify**:
- `src/serial_core/port.rs`
- `src/serial_core/io_loop.rs`

---

### 6. Port Availability Checking
**Status**: Not Implemented
**Impact**: Users get confusing errors when port is in use

**Tasks**:
- [ ] Implement `check_port_availability()` function
- [ ] Add pre-open validation
- [ ] Show which process is using the port (if possible)
- [ ] Add automatic port busy retry with backoff
- [ ] Add port monitoring for hot-plug detection

**Files to modify**:
- `src/serial_core/port.rs`

---

## 🟢 Low Priority

### 7. Windows Registry Integration
**Status**: Not Implemented
**Impact**: Advanced users need registry access for configuration

**Tasks**:
- [ ] Add registry query functions for port settings
- [ ] Add ability to persist port configuration in registry
- [ ] Add device parameters extraction
- [ ] Add COM port database extraction

**Files to create**:
- `src/platform/windows.rs`

---

### 8. Event Log Integration
**Status**: Not Implemented
**Impact**: Difficult to debug issues in production

**Tasks**:
- [ ] Add Windows Event Log tracing
- [ ] Log port open/close operations
- [ ] Log configuration changes
- [ ] Add error reporting to event log

**Files to create**:
- `src/platform/windows/logging.rs`

---

### 9. Device Manager Integration
**Status**: Not Implemented
**Impact**: Advanced device management capabilities

**Tasks**:
- [ ] Add device installation detection
- [ ] Add driver version information
- [ ] Add device capabilities query
- [ ] Add device disable/enable functionality

**Files to create**:
- `src/platform/windows/device.rs`

---

## Completed Tasks

### ✅ Basic Windows Support
- [x] Cross-platform compilation support
- [x] Basic serial port operations
- [x] COM port name recognition
- [x] APPDATA configuration directory handling
- [x] Basic error handling for Windows

---

## Testing Requirements

### Unit Tests
- [ ] Flow control configuration tests
- [ ] DTR/RTS signal control tests
- [ ] Enhanced error message tests
- [ ] Port enumeration tests with mock data

### Integration Tests
- [ ] Real hardware flow control tests
- [ ] DTR/RTS control with actual devices
- [ ] Port enumeration on Windows with various devices
- [ ] Permission error handling tests

### Documentation
- [ ] Update USAGE.md with Windows-specific examples
- [ ] Add troubleshooting section for Windows
- [ ] Add flow control usage examples
- [ ] Add DTR/RTS usage examples

---

## Dependencies

### Potential New Dependencies
- **winreg**: For Windows registry access
- **windows**: For Windows-specific APIs (optional, for advanced features)

### Current Dependencies
- `tokio-serial` (5.4.5) - Already supports Windows
- `serialport` (4.9.0) - Already supports Windows

---

## Milestones

### v0.2.0 - Windows Enhancements Phase 1
- Flow control support
- DTR/RTS signal control
- Enhanced error messages
- Enhanced port enumeration

### v0.3.0 - Windows Enhancements Phase 2
- Port availability checking
- Windows-specific timeouts
- Basic registry integration

### v0.4.0 - Advanced Windows Features
- Event log integration
- Device manager integration
- Advanced diagnostics

---

## Notes

- All changes must maintain cross-platform compatibility
- Windows-specific code should use `cfg(windows)` attribute
- All new features must have corresponding tests
- All new features must be documented in USAGE.md

---

**Last Updated**: 2026-04-02
**Maintainer**: Serial CLI Team
