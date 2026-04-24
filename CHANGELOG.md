# Changelog
All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-04-24 ✅ RELEASED

### 🎉 Virtual Port Backend Architecture

- **Pluggable backend system** - `VirtualBackend` trait enables extensible backends
- **PTY Backend** (Unix/macOS) - Refactored from existing code, improved performance
- **NamedPipe Backend** (Windows) - Native Windows named pipes implementation
- **Socat Backend** (Cross-platform) - Socat-based virtual ports with auto-detection
- **Platform auto-detection** - Automatically selects best backend (PTY on Unix, NamedPipe on Windows)
- **BackendFactory** - Priority chain: CLI flag → config file → auto-detection
- **Config integration** - `virtual.backend` setting for default backend
- **CLI enhancement** - `--backend` flag on `virtual create` command
- **Error handling** - New error types: `UnsupportedBackend`, `MissingDependency`, `BackendInitFailed`
- **Helpful error messages** - Installation hints for missing dependencies (socat)

### Architecture

- Created `serial_core/backends/` module with trait-based design
- `VirtualBackend` trait: `create_pair()`, `is_healthy()`, `get_stats()`, `cleanup()`
- Backend implementations: `PtyBackend`, `NamedPipeBackend`, `SocatBackend`
- Runtime polymorphism via `Box<dyn VirtualBackend>`
- Backward compatibility maintained via type aliases

### Usage

```bash
# Auto-detect (recommended)
serial-cli virtual create

# Explicit backend selection
serial-cli virtual create --backend pty
serial-cli virtual create --backend socat
serial-cli virtual create --backend namedpipe

# Set default in config
serial-cli config set virtual.backend socat
```

### Documentation

- Updated README.md with virtual port examples
- Added backend installation guide (socat dependencies)
- Updated feature list and troubleshooting section
- Added design spec: `docs/superpowers/specs/2026-04-24-virtual-port-backends-design.md`

### Testing

- All 214 tests passing
- Added unit tests for backend implementations
- Added BackendType parsing and detection tests
- Property-based tests for backend selection

---

## [0.3.0] - 2026-04-24

### Sniffing — Session Management

- **`sniff start`** now spawns a background daemon process, freeing the parent shell for further commands
- **`sniff stop`** — gracefully stops an active sniff session (SIGTERM → SIGKILL fallback)
- **`sniff stats`** — shows port, PID, uptime, and config for the active session
- **`sniff save`** — saves captured packets from the session's output file to a specified path
- Session registry uses file-based state (PID + config in cache dir) with stale session auto-cleanup
- Cross-platform process management (Unix via libc, Windows via Win32 API)

### Batch Processing — Enhanced

- **Variable substitution** — `${VAR}` and `$VAR` syntax in script paths and `set` values, with environment variable fallback
- **`set KEY value`** directive — define variables within batch files
- **Loop blocks** — `loop N` ... `end` with validated parsing (detects unclosed loops, unexpected `end`, nested loops)
- **`sleep MS`** directive — add delays between script executions
- **`batch list`** — now searches current directory + `~/.config/serial_cli/` for `.batch`, `.txt`, `.lua` files
- **Error reporting** — per-script error messages displayed in batch summary output

### Fixes

- Fixed daemon pipe leak: explicitly close stdin/stdout/stderr handles before `std::mem::forget(child)`, preventing blocking writes under load
- Removed unused `_display` parameter from sniff daemon (reduces CLI surface)

---

## [0.2.0] - 2025-04-09

### 🎉 Major Features - GUI Application Complete

#### Frontend (React + Tauri)
- ✅ **Complete UI Overhaul** - Cyber-industrial aesthetic design
- ✅ **Serial Port Management** - Full port configuration, open/close, status monitoring
- ✅ **Real-time Data Monitoring** - Live data display with RX/TX distinction
- ✅ **Lua Script Editor** - Monaco Editor integration with syntax highlighting
- ✅ **Protocol Management** - Built-in and custom protocol loading with validation
- ✅ **Settings System** - Comprehensive configuration with persistence
- ✅ **Data Export** - TXT/CSV/JSON formats with filtering options
- ✅ **System Notifications** - Cross-platform desktop notifications
- ✅ **Keyboard Shortcuts** - Command palette and global shortcuts
- ✅ **Data Persistence** - Auto-save for settings, scripts, protocols, and recent ports

#### Backend (Tauri Commands)
- ✅ **Serial Port Commands** - list_ports, open_port, close_port, get_port_status
- ✅ **Data Transfer** - send_data, read_data with event emission
- ✅ **Script Execution** - execute_script with real LuaJIT runtime
- ✅ **Protocol Management** - load_protocol, validate_protocol, list_protocols
- ✅ **Configuration** - get_config, update_config
- ✅ **Window Control** - show_window, hide_window, toggle_window

#### Design System
- ✅ **Icon System** - lucide-react SVG icons (replaced emoji)
- ✅ **Color Scheme** - signal (green), alert (red), amber (yellow), info (blue)
- ✅ **Typography** - Instrument Sans, JetBrains Mono, Instrument Serif
- ✅ **Animations** - fade-in, slide-up, pulse-slow transitions
- ✅ **Components** - Panel, Toast, CommandPalette with consistent styling

### Technical Achievements
- ✅ **Type Safety** - 100% TypeScript strict mode compliance
- ✅ **State Management** - Context-based with proper separation of concerns
- ✅ **Event System** - Real-time data flow with Tauri events
- ✅ **Persistence** - localStorage integration for all user data
- ✅ **Error Handling** - Comprehensive error catching and user feedback
- ✅ **Performance** - Optimized rendering and data handling

### Breaking Changes
- None (backward compatible)

### Known Issues
- None (production ready)

---

## [0.1.0] - 2025-04-01

### Features
- Initial release of Serial CLI
- Core serial port management
- Lua scripting support
- Modbus RTU/ASCII protocols
- AT Command protocol
- Interactive CLI mode
- Batch execution mode
- JSON output format

### Bug Fixes
- Initial implementation

### Documentation
- Initial documentation setup
