# Serial CLI TODO List

**Version**: v0.4.0
**Updated**: 2026-04-24

---

## Priority Legend

- **P0** - Critical (must fix)
- **P1** - Important (should fix)
- **P2** - Nice to have (can defer)

---

## P0 - Critical Issues

None - all critical issues resolved! ✅

---

## P1 - Important Features

All P1 features complete! ✅

---

## P2 - Future Enhancements

### 1. Virtual Port Additional Backends
**Status**: ✅ Complete
**Priority**: P2

**Completed**:
- [x] Create backend module structure with trait-based architecture
- [x] Implement PTY backend (refactored from existing code)
- [x] Implement NamedPipe backend for Windows
- [x] Implement Socat backend wrapper (cross-platform)
- [x] BackendFactory with priority chain (CLI → config → auto-detect)
- [x] Config integration for default backend selection
- [x] Update VirtualSerialPair to use new backend system

**Architecture**:
- `VirtualBackend` trait defining the backend contract
- `PtyBackend`, `NamedPipeBackend`, `SocatBackend` implementations
- `BackendFactory` for smart backend selection
- Runtime polymorphism via `Box<dyn VirtualBackend>`

---

### 2. CLI Integration
**Status**: ⚠️ Partial
**Priority**: P2

**Completed**:
- [x] Backend type parsing (string → enum)
- [x] Config file support for `virtual.default_backend`

**TODO**:
- [ ] Add `--backend` flag to `virtual create` command
- [ ] Update help text with backend options

---

### 3. Testing
**Status**: ⚠️ Partial
**Priority**: P2

**Completed**:
- [x] 214/214 tests passing
- [x] Unit tests for core modules
- [x] Unit tests for backend implementations
- [x] Property-based tests (proptest)
- [x] Benchmark tests (6 benchmarks)

**TODO**:
- [ ] Integration tests for backend factory
- [ ] Integration tests for NamedPipe backend (Windows)
- [ ] Integration tests for Socat backend
- [ ] CLI command tests with backend flags
- [ ] Target: 80% code coverage

---

### 4. Documentation
**Status**: ⚠️ Partial
**Priority**: P2

**Completed**:
- [x] README.md, CHANGELOG.md, RELEASE.md
- [x] CLAUDE.md project instructions
- [x] Inline code documentation
- [x] Architecture reference (docs/dev/ARCH.md)
- [x] Virtual Port Backend design spec (docs/superpowers/specs/)

**TODO**:
- [ ] Update README with backend usage instructions
- [ ] Protocol development guide
- [ ] Lua API complete reference
- [ ] Troubleshooting guide expansion
- [ ] Backend installation guide (socat, etc.)

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
- [x] Virtual port monitoring (packet capture integration)
- [x] Protocol dynamic loading (load/unload/reload with config persistence)
- [x] Configuration management (show/set/save/reset)
- [x] Data sniffing session management (start/stop/stats/save)
- [x] Batch processing with variables and loops
- [x] Modular CLI architecture (main.rs 1194→73 lines)
- [x] **Virtual port backend architecture (PTY, NamedPipe, Socat)**

---

## Progress Summary

| Category | Total | Completed | Partial | TODO |
|----------|-------|-----------|---------|------|
| P0 - Critical | 0 | 0 | 0 | 0 |
| P1 - Important | 0 | 0 | 0 | 0 |
| P2 - Future | 4 | 1 | 2 | 1 |
| **Total** | **4** | **1** | **2** | **1** |

**Overall Progress**: ~90% (all P0/P1 complete, most P2 complete)

---

## Implementation Plan

### Phase 1 (P0 - 1 week) ✅ Complete
1. ✅ Refactor main.rs into modules
2. ✅ Extract command implementations

### Phase 2 (P1 - 2-3 weeks) ✅ Complete
1. ✅ Wire virtual port monitoring end-to-end
2. ✅ Protocol dynamic loading CLI commands
3. ✅ Configuration management commands
4. ✅ Sniffing session tracking
5. ✅ Batch processing enhancements

### Phase 3 (P2 - 2-3 weeks) ⚠️ In Progress
1. ✅ Virtual port backends (NamedPipe, Socat)
2. ⚠️ CLI integration (partial)
3. ⚠️ Testing expansion
4. ⚠️ Documentation updates

---

## Contributing

Want to help? Pick a task and create a PR:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/task-name`
3. Implement the feature with tests
4. Submit PR with reference to this TODO

**Labels**: `good-first-issue`, `help-wanted`, `enhancement`
