# Serial CLI TODO List

**Version**: v0.5.0
**Updated**: 2026-04-24

---

## Priority Legend

- **P0** - Critical (must fix)
- **P1** - Important (should fix)
- **P2** - Nice to have (can defer)

---

## P0 - Critical Issues

### 1. DTR/RTS Hardware Signal Control — Completely Non-functional
**Status**: ✅ Fixed
**Priority**: P0
**Files**: `src/serial_core/port.rs`

Port type changed from `Box<dyn tokio_serial::SerialPort>` to `Box<dyn serialport::SerialPort>`.
- Unix: opens via `serialport::new().open_native()` → `TTYPort`, sets DTR/RTS via `TIOCMGET`/`TIOCMSET` ioctl on raw fd
- Windows: opens via `serialport::new().open_native()` → `COMPort`
- `UnixSignalController` stored in `SerialPortHandle` for runtime DTR/RTS changes
- Removed `MioSerialPort` wrapper (mio_serial not in dependencies)

### 2. ProtocolWatcher Event Channel — Events Never Delivered
**Status**: ✅ Fixed
**Priority**: P0
**Files**: `src/protocol/watcher.rs`

- Stored `reload_tx` (cloned before closure) and `reload_rx` as struct fields
- `reload_events()` now returns `Some(actual_receiver)` via `Option::take()`
- Added `sender()` method for cloning the sender

### 3. Sniff Daemon — No Internal Read Loop
**Status**: 🔴 Not Started
**Priority**: P0
**Files**: `src/cli/sniff_session.rs`, `src/serial_core/sniffer.rs`

`run_sniff_daemon` calls `start_sniffing(port)` and immediately loops waiting for ctrl_c. The SnifferSession has no internal loop that reads from the serial port. Packet capture infrastructure exists but is never driven.

- [ ] Add async read loop in sniff daemon that periodically reads from the port
- [ ] Feed captured data into the sniffer's packet capture

---

## P1 - Important Issues

### 4. Lua Bindings Stub Functions
**Status**: 🔴 Not Started
**Priority**: P1
**Files**: `src/lua/bindings.rs`

| API | Problem |
|-----|---------|
| `virtual_stop()` | Returns `(true, "...")` without doing anything |
| `protocol_unload()` | Returns success without unloading |
| `protocol_reload()` | Returns success without reloading |

- [ ] `virtual_stop`: integrate with a global virtual pair registry
- [ ] `protocol_unload`: delegate to ConfigManager.remove_custom_protocol
- [ ] `protocol_reload`: delegate to ConfigManager.update_custom_protocol

### 5. VirtualSerialPair NamedPipe/Socat Backends Not Accessible via CLI
**Status**: 🔴 Not Started
**Priority**: P1
**Files**: `src/serial_core/virtual_port.rs`

`VirtualSerialPair::create()` returns errors for NamedPipe and Socat: "not yet implemented via old API". The new `BackendFactory` + `VirtualBackend` trait supports these, but the CLI `virtual create` command uses the old API.

- [ ] Refactor VirtualSerialPair::create to use BackendFactory for all backends
- [ ] Or integrate new backend API into the virtual command handler

### 6. Benchmark Virtual Port — Simulated Instead of Real
**Status**: 🔴 Not Started
**Priority**: P1
**Files**: `src/benchmark/runner.rs`

`run_virtual_port_benchmarks()` uses `std::thread::sleep(Duration::from_micros(100))` instead of actually creating PTY pairs.

- [ ] Create real PTY pairs with timeout for benchmark measurement
- [ ] Handle platform-specific availability gracefully

### 7. Benchmark Save/Load Uses Text Format (Not JSON)
**Status**: ✅ Fixed
**Priority**: P1
**Files**: `src/cli/commands/benchmark.rs`, `src/benchmark/mod.rs`, `src/benchmark/reporter.rs`

- Added `serde::Serialize/Deserialize` derives for `BenchmarkCategory`, `BenchmarkResult`, `BenchmarkReport`
- Replaced text-format save with `serde_json::to_string_pretty`
- Replaced custom parser with `serde_json::from_str`

### 8. NamedPipe Backend — Handle Leak
**Status**: 🔴 Not Started
**Priority**: P1
**Files**: `src/serial_core/backends/named_pipe.rs`

Named pipe handles are "intentionally leaked" — not stored for cleanup. `cleanup()` does nothing. `is_healthy()` only checks a bool flag.

- [ ] Store handles properly for cleanup
- [ ] Implement real health check
- [ ] Implement proper resource cleanup on drop

---

## P2 - Future Enhancements

### 9. Performance Optimization (v0.5.0)
**Status**: 🚧 In Progress

**Completed**:
- [x] Add benchmark module structure
- [x] Implement BenchmarkRunner with timing and throughput measurement
- [x] Implement BenchmarkReporter for result comparison
- [x] Add BenchmarkCommand to CLI (run, compare, list)
- [x] Create basic serial I/O benchmarks
- [x] Create virtual port benchmarks (creation timing)
- [x] Create protocol benchmarks (parsing throughput)
- [x] Implement benchmark result persistence (save/load)
- [x] Implement benchmark comparison (regression detection)
- [x] Add startup time benchmarks

**In Progress**:
- [ ] Add memory usage benchmarks
- [ ] Add concurrency benchmarks
- [ ] Optimize data transfer based on benchmark findings

**Pending**:
- [ ] Zero-copy data transfer optimization
- [ ] AsyncFd polling optimization
- [ ] Buffer size tuning
- [ ] Batch read/write optimization
- [ ] Lazy initialization for faster startup
- [ ] Memory pool for buffer reuse

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
- [x] **Protocol hot-reload management (enable/disable/status)**
- [x] **Benchmark infrastructure (runner, reporter, CLI integration)**
- [x] **Startup time benchmarks (cold/warm start, protocol load, Lua init)**

---

## Progress Summary

| Category | Total | Completed | Partial | TODO |
|----------|-------|-----------|---------|------|
| P0 - Critical | 3 | 0 | 0 | 3 |
| P1 - Important | 5 | 0 | 1 | 5 |
| P2 - Future | 4 | 4 | 0 | 0 |
| **Total** | **12** | **4** | **1** | **8** |

**Overall Progress**: 🚧 ~75% complete, 8 functional gaps identified

---

## Implementation Plan

### Phase 1 (P0 - Critical Fixes)
1. Fix DTR/RTS hardware signal control (Unix ioctl + Windows EscapeCommFunction)
2. Fix ProtocolWatcher event channel (store tx/rx properly)
3. Fix sniff daemon read loop (add async polling in daemon)

### Phase 2 (P1 - Important Fixes)
1. Implement Lua binding stubs (virtual_stop, protocol_unload, protocol_reload)
2. Wire NamedPipe/Socat backends into VirtualSerialPair::create
3. Fix benchmark virtual port (real PTY instead of sleep)
4. Implement JSON serialization for benchmark save/load
5. Fix NamedPipe handle leak

### Phase 3 (P2 - Optimization)
1. Memory usage benchmarks
2. Concurrency benchmarks
3. Performance optimization based on findings
