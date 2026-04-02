# Changelog - Windows Platform Enhancements

## [0.1.0] - 2026-04-02

### Added - Windows Platform Improvements

#### Serial Port Configuration
- **Flow Control Support**: Added hardware (RTS/CTS) and software (XON/XOFF) flow control
  - New `FlowControl` enum: `None`, `Software`, `Hardware`
  - Configurable via Lua API: `flow_control = "hardware"`

- **Signal Control**: Added DTR and RTS signal configuration
  - New fields: `dtr_enable`, `rts_enable` in `SerialConfig`
  - Default: both enabled for compatibility

#### Error Handling
- **Enhanced Error Messages**: Added helpful suggestions for common Windows issues
  - "Access is denied" error detection
  - Permission errors include resolution suggestions
  - Port busy errors with troubleshooting tips

- **Better Windows Error Detection**:
  - "Access is denied" (Windows permission errors)
  - "The system cannot find the file" (port not found)
  - "used by another application" (port busy)

#### Port Information
- **Enhanced Port Enumeration**: Extended `SerialPortInfo` structure
  - `friendly_name`: Human-readable port name
  - `hardware_id`: Device hardware identifier
  - `manufacturer`: Device manufacturer
  - `com_number`: COM port number (Windows-specific)

#### Lua API Changes
- **Updated `serial_open()`**: Now accepts configuration table
  ```lua
  -- Old API (still works)
  serial_open("COM3", 115200)

  -- New API (recommended)
  serial_open("COM3", {
      baudrate = 115200,
      data_bits = 8,
      stop_bits = 1,
      parity = "none",
      flow_control = "hardware",
      timeout = 1000,
      dtr_enable = true,
      rts_enable = true
  })
  ```

### Documentation
- **TODO_WINDOWS.md**: Roadmap for Windows-specific improvements
- **WINDOWS_EXAMPLES.md**: Comprehensive Windows usage guide with examples
- **WINDOWS_IMPROVEMENTS.md**: Summary of completed enhancements
- **examples/windows_serial_example.lua**: Example Lua script for Windows

### Technical Details
- Modified `src/serial_core/port.rs`:
  - Added `FlowControl` enum
  - Extended `SerialConfig` with new fields
  - Enhanced `SerialPortInfo` structure
  - Improved error detection for Windows

- Modified `src/error.rs`:
  - Added `permission_denied()` and `port_busy()` helper methods
  - Enhanced error messages with suggestions

- Modified `src/lua/bindings.rs`:
  - Updated `serial_open()` to accept configuration table
  - Added support for new configuration options

### Testing
- All 77 existing tests passing
- Updated tests for new Lua API
- No breaking changes to existing functionality

### Compatibility
- ✅ Fully backward compatible
- ✅ Cross-platform maintained
- ✅ No changes to existing API (old API still works)
- ✅ Windows, Linux, macOS support maintained

---

## [Future Versions]

### v0.2.0 - Planned
- Runtime DTR/RTS control methods
- Windows registry integration
- Port availability checking
- Advanced timeout configuration

### v0.3.0 - Planned
- Windows Event Log integration
- Device Manager integration
- Advanced diagnostics tools

### v0.4.0 - Planned
- GUI configuration tool
- Real-time monitoring
- Virtual COM port support

---

**For detailed information, see**:
- [TODO_WINDOWS.md](TODO_WINDOWS.md) - Roadmap
- [WINDOWS_EXAMPLES.md](WINDOWS_EXAMPLES.md) - Usage guide
- [WINDOWS_IMPROVEMENTS.md](WINDOWS_IMPROVEMENTS.md) - Implementation details
