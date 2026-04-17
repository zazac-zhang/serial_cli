# Serial CLI TODO List

**Version**: v0.2.0
**Updated**: 2026-04-17

---

## Priority Legend

- **P0** - Critical (must fix)
- **P1** - Important (should fix)
- **P2** - Nice to have (can defer)

---

## P0 - Critical Issues

### 1. Code Architecture: Refactor main.rs (1200+ lines)
**Status**: 🔴 Critical
**Impact**: Maintainability, Testing

The main.rs file is too large and violates single responsibility principle.

**Action Items**:
- [ ] Move command implementations to `src/cli/commands/` module
  - [ ] `info.rs` - list, status commands
  - [ ] `exec.rs` - exec, run commands
  - [ ] `session.rs` - shell, open commands
  - [ ] `management.rs` - protocol, sniff, batch, config commands
- [ ] Extract protocol registration to `src/protocol/registration.rs`
- [ ] Create `src/cli/parsers.rs` for hex/base64 utilities
- [ ] Keep main.rs under 200 lines (entry point only)

**Target**: main.rs < 200 lines

---

## P1 - Important Features

### 2. Protocol Dynamic Loading
**Status**: ⚠️ Partial
**Priority**: P1

**Completed**:
- [x] `protocol validate` - Validate protocol scripts
- [x] `protocol list` - List all protocols
- [x] `protocol info` - Show protocol info

**TODO**:
- [ ] `protocol load` - Load custom protocol from Lua script
- [ ] `protocol unload` - Unload custom protocol
- [ ] `protocol reload` - Reload protocol from disk

**Current State**: Commands print "will be implemented in next version"

---

### 3. Configuration Management
**Status**: ⚠️ Partial
**Priority**: P1

**Completed**:
- [x] `config show` - Display configuration (hardcoded)

**TODO**:
- [ ] `config set` - Set configuration value
- [ ] `config save` - Save configuration to file
- [ ] `config reset` - Reset to defaults

**Current State**: Commands print "will be implemented in next version"

---

### 4. Batch Processing
**Status**: ⚠️ Partial
**Priority**: P1

**Completed**:
- [x] `batch run script.lua` - Run single script
- [x] `batch run batch.txt` - Run batch file (basic)

**TODO**:
- [ ] `batch list` - List available batch files
- [ ] Support comments in batch files
- [ ] Support variables and loops
- [ ] Better error handling and reporting

---

### 5. Data Sniffing
**Status**: ⚠️ Basic
**Priority**: P2

**Completed**:
- [x] `sniff start` - Start packet capture

**TODO**:
- [ ] `sniff stop` - Stop active sniffing session
- [ ] `sniff stats` - Show capture statistics
- [ ] `sniff save` - Save captured packets
- [ ] Session tracking and management

---

## P2 - Future Enhancements

### 6. Testing
**Status**: ⚠️ Partial
**Priority**: P1

- [ ] Add unit tests for command execution
- [ ] Add integration tests for CLI commands
- [ ] Add tests for protocol loading
- [ ] Target: 80% code coverage

---

### 7. Documentation
**Status**: ✅ Good
**Priority**: P1

- [ ] Add protocol development guide
- [ ] Add Lua API complete reference
- [ ] Add troubleshooting guide expansion
- [ ] Add video tutorials

---

## Completed ✅

- [x] Basic serial port communication
- [x] Lua scripting engine
- [x] Interactive shell (REPL)
- [x] Built-in protocols (Modbus, AT Command, Line)
- [x] Data format support (text, hex, base64)
- [x] Error handling with thiserror
- [x] CLI command structure
- [x] GUI application (Tauri + React)

---

## Implementation Plan

### Phase 1 (P0 - 1 week)
1. Refactor main.rs into modules
2. Extract command implementations
3. Create proper module structure

### Phase 2 (P1 - 2-3 weeks)
1. Implement protocol dynamic loading
2. Complete configuration management
3. Enhance batch processing

### Phase 3 (P2 - 1-2 weeks)
1. Advanced sniffing features
2. Testing and documentation
3. Performance optimizations

---

## Progress Summary

| Category | Total | Completed | In Progress | TODO |
|----------|-------|-----------|-------------|------|
| P0 - Critical | 1 | 0 | 0 | 1 |
| P1 - Important | 4 | 0 | 4 | 0 |
| P2 - Future | 2 | 1 | 1 | 0 |
| **Total** | **7** | **1** | **5** | **1** |

**Overall Progress**: 14% complete

---

## Contributing

Want to help? Pick a task and create a PR:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/task-name`
3. Implement the feature with tests
4. Submit PR with reference to this TODO

**Labels**: `good-first-issue`, `help-wanted`, `enhancement`

---

## Resources

- [Development Guide](DEVELOPMENT.md)
- [Code Review](CODE_REVIEW.md)
- [Project Instructions](CLAUDE.md)
