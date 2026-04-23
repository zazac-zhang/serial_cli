# Serial CLI TODO List

**Version**: v0.2.0
**Updated**: 2026-04-23

---

## Priority Legend

- **P0** - Critical (must fix)
- **P1** - Important (should fix)
- **P2** - Nice to have (can defer)

---

## P0 - Critical Issues

### 1. Code Architecture: Refactor main.rs
**Status**: ✅ Complete
**Impact**: Maintainability, Testing

Refactored main.rs from 1194 lines to 73 lines.

**Completed**:
- [x] Create `src/cli/args.rs` - Cli, Commands structs (clap definitions)
- [x] Add `VirtualCommand` to `src/cli/types.rs` (all command enums unified)
- [x] Create `src/cli/commands/protocol.rs` - protocol command handler
- [x] Create `src/cli/commands/sniff.rs` - sniff command handler
- [x] Create `src/cli/commands/batch.rs` - batch command handler
- [x] Create `src/cli/commands/config.rs` - config command handler
- [x] Create `src/cli/commands/virtual_port.rs` - virtual port handler + registry
- [x] Create `src/cli/commands/ports.rs` - list_ports, send_data
- [x] Create `src/cli/commands/script.rs` - run_lua_script (uses LuaStdLib)
- [x] Fix `src/cli/commands/parsers.rs` - doc tests and clippy warnings
- [x] main.rs reduced to 73 lines (entry point + dispatch only)

**Bonus**: Eliminated duplicate `register_stdlib_utils()` in main.rs (replaced by `LuaStdLib::register_all_on()`)

---

## P1 - Important Features

### 2. Virtual Port Monitoring (End-to-End)
**Status**: ⚠️ Partial (~30%)
**Priority**: P1

Backend capture infrastructure exists, but not wired through to frontend.

**Completed**:
- [x] `CapturedPacket` / `PacketDirection` / `PacketCapture` structs in Rust
- [x] `captured_packets()` and `is_monitoring()` public methods
- [x] `VirtualStats` extended with capture fields
- [x] Frontend monitoring checkbox UI (disabled with "coming soon")

**TODO**:
- [ ] Integrate packet capture into bridge task (record each read with direction + payload)
- [ ] Add Tauri command to fetch captured packets
- [ ] Enable monitoring checkbox in GUI, wire to backend
- [ ] Real-time traffic display in virtual port panel
- [ ] Packet filtering and search

**Known Limitation**: "Virtual port monitoring is limited... For full monitoring, use regular serial ports"

---

### 3. Protocol Dynamic Loading
**Status**: ✅ Complete (except hot-reload)
**Priority**: P1

**Completed**:
- [x] `protocol validate` - Validate protocol scripts
- [x] `protocol list` - List all protocols (built-in + custom)
- [x] `protocol info` - Show protocol info (built-in + custom)
- [x] `protocol load` - Load + validate + persist to config
- [x] `protocol unload` - Remove from config
- [x] `protocol reload` - Re-validate + update config
- [x] ConfigManager helpers for custom protocol CRUD
- [x] Protocol persistence via config file (`protocols.custom`)
- [x] ProtocolManager fully implemented with load/unload/reload
- [x] ProtocolLoader with validation
- [x] ProtocolRegistry with factory pattern

**TODO**:
- [ ] Implement hot-reloading (file watcher)

---

### 4. Configuration Management
**Status**: ✅ Complete
**Priority**: P1

**Completed**:
- [x] `config show` - Display full configuration (text/JSON)
- [x] `config set` - Set configuration value with validation
- [x] `config save` - Save configuration to file
- [x] `config reset` - Reset to defaults
- [x] ConfigManager with load/validate/set/save
- [x] TOML-based configuration with fallback defaults

---

### 5. Data Sniffing Session Management
**Status**: ⚠️ Partial (~70%)
**Priority**: P1

**Completed**:
- [x] `sniff start` - Start packet capture with all options
- [x] Real-time packet capture and display
- [x] File output support
- [x] TX/RX direction tracking

**TODO**:
- [ ] Track active sniffing sessions (state management)
- [ ] `sniff stop` - Stop active sniffing session
- [ ] `sniff stats` - Show capture statistics (currently "No active sniff session")
- [ ] `sniff save` - Save captured packets (requires active session)

---

### 6. Batch Processing
**Status**: ⚠️ Partial (~60%)
**Priority**: P1

**Completed**:
- [x] `batch run script.lua` - Run single script
- [x] `batch run batch.txt` - Run batch file
- [x] Comment filtering (lines starting with #)
- [x] Concurrent execution control
- [x] Progress tracking and error reporting

**TODO**:
- [ ] `batch list` - List available batch files
- [ ] Support variable substitution
- [ ] Support loops
- [ ] Better error recovery

---

## P2 - Future Enhancements

### 7. Virtual Port Additional Backends
**Status**: ❌ Not Started
**Priority**: P2

Only PTY backend (Unix/macOS) is functional. NamedPipe and Socat options have been removed from GUI.

**TODO**:
- [ ] NamedPipe backend (Windows support)
- [ ] Socat backend (cross-platform alternative)

---

### 8. Testing
**Status**: ⚠️ Partial
**Priority**: P2

- [x] 58/58 tests passing
- [x] Unit tests for core modules
- [x] Integration tests for virtual ports
- [x] Property-based tests (proptest)
- [x] Benchmark tests (6 benchmarks)
- [ ] Add unit tests for command execution
- [ ] Add integration tests for CLI commands
- [ ] Add tests for protocol loading
- [ ] Target: 80% code coverage

---

### 9. Documentation
**Status**: ⚠️ Partial
**Priority**: P2

- [x] README.md, CHANGELOG.md, RELEASE.md
- [x] CLAUDE.md project instructions
- [x] Inline code documentation
- [ ] Protocol development guide
- [ ] Lua API complete reference
- [ ] Troubleshooting guide expansion

---

## Completed

- [x] Basic serial port communication (open, close, configure, send/receive)
- [x] Lua scripting engine (LuaJIT with async support)
- [x] Interactive shell (REPL)
- [x] Built-in protocols (Modbus RTU, Modbus ASCII, AT Command, Line)
- [x] Data format support (text, hex, base64)
- [x] Error handling with thiserror
- [x] CLI command structure (clap)
- [x] GUI application (Tauri + React)
- [x] Virtual serial port pairs (PTY backend)
- [x] Event-driven bridge (tokio AsyncFd, no busy-wait)
- [x] Virtual port health checking and auto-cleanup
- [x] Real-time statistics (bytes, packets, errors, uptime)
- [x] Cyber-industrial UI with Lucide icons
- [x] Monaco Editor integration for Lua scripts
- [x] Data export (TXT/CSV/JSON)
- [x] System notifications
- [x] Keyboard shortcuts and command palette
- [x] Data persistence (localStorage)
- [x] Serial sniffer with packet capture

---

## Implementation Plan

### Phase 1 (P0 - 1 week)
1. Refactor main.rs into modules
2. Extract command implementations
3. Create proper module structure

### Phase 2 (P1 - 2-3 weeks)
1. Wire virtual port monitoring end-to-end
2. Connect protocol dynamic loading CLI commands to ProtocolManager
3. Complete configuration management commands
4. Implement sniffing session tracking
5. Enhance batch processing

### Phase 3 (P2 - 2-3 weeks)
1. Additional virtual port backends (NamedPipe, Socat)
2. Testing and code coverage
3. Documentation expansion

---

## Progress Summary

| Category | Total | Completed | Partial | TODO |
|----------|-------|-----------|---------|------|
| P0 - Critical | 1 | 1 | 0 | 0 |
| P1 - Important | 5 | 2 | 3 | 0 |
| P2 - Future | 3 | 0 | 2 | 1 |
| **Total** | **9** | **3** | **5** | **1** |

**Overall Progress**: ~60% (P0 done, 2/5 P1 complete, core features solid)

---

## Contributing

Want to help? Pick a task and create a PR:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/task-name`
3. Implement the feature with tests
4. Submit PR with reference to this TODO

**Labels**: `good-first-issue`, `help-wanted`, `enhancement`
