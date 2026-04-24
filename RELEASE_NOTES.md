# Serial CLI v0.4.0 Release Notes

**Release Date:** April 24, 2026
**Status:** ✅ Production Ready

---

## 🎉 Major Features

### 1. Virtual Port Backend Architecture

**Pluggable Backend System**
- `VirtualBackend` trait defining the backend contract
- `BackendFactory` with priority chain: CLI flag → config → auto-detect
- Runtime polymorphism via `Box<dyn VirtualBackend>`

**Supported Backends:**
- **PTY Backend** (Unix/macOS) - POSIX pseudo-terminals with event-driven bridge
- **NamedPipe Backend** (Windows) - Native Windows named pipes
- **Socat Backend** (Cross-platform) - Socat-based virtual ports

**Platform Auto-Detection:**
- Unix/macOS → PTY (best performance)
- Windows → NamedPipe (native implementation)

### 2. Enhanced CLI Integration

**New CLI Flag:**
```bash
serial-cli virtual create --backend <auto|pty|namedpipe|socat>
```

**Configuration Support:**
```toml
[protocols]
hot_reload = true  # NEW: Protocol hot-reload management

[virtual]
default_backend = "auto"  # NEW: Default backend selection
```

### 3. Protocol Hot-Reload Management

**New Command:**
```bash
serial-cli protocol hot-reload enable   # Enable automatic reloading
serial-cli protocol hot-reload disable  # Disable automatic reloading
serial-cli protocol hot-reload status   # Show current status
```

### 4. Code Quality Improvements

- Removed deprecated type aliases
- Unified to `BackendType` enum throughout codebase
- Zero compilation warnings
- All 214 tests passing

---

## 📊 Statistics

- **Tests:** 214/214 passing (100%)
- **Platforms:** Linux ✅ | macOS ✅ | Windows ✅
- **Lines of Code:** ~15,000+
- **Modules:** 25+ modules
- **Documentation:** Complete

---

## 🔄 Migration from v0.3.0

**Breaking Changes:** None

**New Features:**
- Virtual port backend selection
- Protocol hot-reload management
- Enhanced configuration options

**Deprecations:**
- `VirtualBackend` type alias removed (use `BackendType` instead)

---

## 📦 Installation

```bash
# From source
cargo install --path .

# Or download pre-built binaries
# Visit: https://github.com/yourusername/serial_cli/releases
```

---

## 🐛 Bug Fixes

- Fixed deprecated type alias warnings
- Cleaned up unused imports
- Improved error messages for backend selection

---

## 🙏 Contributors

Thank you to all contributors who made this release possible!

---

## 📝 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete details.
