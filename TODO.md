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

None - all critical issues resolved! ✅

---

## P1 - Important Features

### Performance Optimization (v0.5.0)
**Status**: 🚧 In Progress
**Priority**: P1

**Completed**:
- [x] Add benchmark module structure (src/benchmark/)
- [x] Implement BenchmarkRunner with timing and throughput measurement
- [x] Implement BenchmarkReporter for result comparison
- [x] Add BenchmarkCommand to CLI (run, compare, list)
- [x] Create basic serial I/O benchmarks (buffer copy throughput)
- [x] Create virtual port benchmarks (creation timing)
- [x] Create protocol benchmarks (parsing throughput)
- [x] Implement benchmark result persistence (save/load)
- [x] Implement benchmark comparison (regression detection)

**In Progress**:
- [ ] Enhance benchmarks with real serial I/O operations
- [x] Add startup time benchmarks
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

**Usage**:
```bash
# List available benchmarks
serial-cli benchmark list

# Run specific benchmark category
serial-cli benchmark run serial-io --iterations 1000

# Run all benchmarks
serial-cli benchmark run all --output results.txt

# Compare benchmark results
serial-cli benchmark compare baseline.txt current.txt
```

---

## P2 - Future Enhancements

All P1 features complete! ✅

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
| P0 - Critical | 0 | 0 | 0 | 0 |
| P1 - Important | 1 | 0 | 1 | 0 |
| P2 - Future | 4 | 4 | 0 | 0 |
| **Total** | **5** | **4** | **1** | **0** |

**Overall Progress**: 🚧 80% (v0.5.0 in progress)

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

### Phase 3 (P2 - 2-3 weeks) ✅ Complete
1. ✅ Virtual port backends (NamedPipe, Socat)
2. ✅ CLI integration (backend selection)
3. ✅ Testing expansion
4. ✅ Documentation updates

### Phase 4 (P1 - v0.5.0) 🚧 In Progress
1. 🚧 Benchmark infrastructure (basic implementation complete)
2. ⏳ Performance optimization based on benchmark findings
3. ⏳ Data transfer optimization
4. ⏳ Startup time optimization
5. ⏳ Memory usage optimization

---

## Contributing

Want to help? Pick a task and create a PR:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/task-name`
3. Implement the feature with tests
4. Submit PR with reference to this TODO

**Labels**: `good-first-issue`, `help-wanted`, `enhancement`
