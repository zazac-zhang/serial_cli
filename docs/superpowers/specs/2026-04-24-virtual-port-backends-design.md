# Virtual Port Backend Architecture Design

**Date**: 2026-04-24
**Status**: Approved
**Priority**: P2 (Future Enhancement)

---

## Overview

Extend the virtual serial port system to support multiple backends with pluggable architecture:
- **PTY Backend** (existing) - Unix/macOS pseudo-terminals
- **NamedPipe Backend** (new) - Windows named pipes
- **Socat Backend** (new) - Cross-platform via socat binary

The design uses a trait-based factory pattern for clean extensibility and runtime backend selection.

---

## Architecture

### Directory Structure

```
src/serial_core/
├── virtual_port.rs           # Existing: VirtualSerialPair, registry
├── backends/
│   ├── mod.rs                # Backend exports, BackendType enum
│   ├── trait.rs              # VirtualBackend trait definition
│   ├── pty.rs                # PTY backend (refactored from existing code)
│   ├── named_pipe.rs         # Windows NamedPipe backend
│   └── socat.rs              # Socat backend wrapper
└── factory.rs                # BackendFactory (auto-detect, config, CLI)
```

### Component Responsibilities

| Component | Responsibility |
|-----------|---------------|
| `VirtualBackend` trait | Core contract: create_pair(), is_healthy(), get_stats(), cleanup() |
| Backend implementations | Own lifecycle, cleanup, and stats collection |
| `BackendFactory` | Backend selection with priority: CLI → config → auto-detect |
| `VirtualSerialPair` | Updated to use `Box<dyn VirtualBackend>` for runtime polymorphism |

---

## Core Trait Definition

```rust
#[async_trait]
pub trait VirtualBackend: Send + Sync {
    /// Create a virtual serial pair
    async fn create_pair(&mut self) -> Result<(VirtualPortEnd, VirtualPortEnd)>;
    
    /// Check if the backend is healthy/running
    async fn is_healthy(&self) -> bool;
    
    /// Get runtime statistics
    async fn get_stats(&self) -> BackendStats;
    
    /// Get backend type identifier
    fn backend_type(&self) -> &'static str;
    
    /// Clean up resources
    async fn cleanup(&mut self) -> Result<()>;
}
```

**Supporting types:**

```rust
pub struct VirtualPortEnd {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct BackendStats {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub uptime_seconds: u64,
}
```

---

## Backend Selection

### BackendType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Auto,           // Platform auto-detection
    Pty,            // PTY (Unix/macOS)
    NamedPipe,      // NamedPipe (Windows)
    Socat,          // Socat (cross-platform)
}
```

### Priority Chain

```
1. CLI flag (--backend pty|namedpipe|socat)
2. Config file setting (virtual.default_backend)
3. Platform auto-detection
   - Windows → NamedPipe
   - Unix/macOS → PTY
```

### Auto-Detection Logic

```rust
impl BackendType {
    pub fn detect() -> Self {
        #[cfg(windows)]
        return BackendType::NamedPipe;
        
        #[cfg(not(windows))]
        return BackendType::Pty;
    }
}
```

---

## Configuration

### Config File Schema

```toml
# ~/.config/serial_cli/config.toml

[virtual]
default_backend = "auto"  # or "pty", "namedpipe", "socat"
```

### CLI Integration

```bash
# Auto-detect (default)
serial-cli virtual create

# Explicit backend selection
serial-cli virtual create --backend pty
serial-cli virtual create --backend socat
serial-cli virtual create --backend namedpipe

# Set default in config
serial-cli config set virtual.default_backend socat
```

---

## Backend Implementations

### PTY Backend (Refactored)

- Extract existing PTY code into `PtyBackend`
- Implement `VirtualBackend` trait
- Unix/macOS only (compile-time guard)

### NamedPipe Backend (New)

**Platform:** Windows only

**Implementation approach:**
- Create two named pipes: `\\.\pipe\serial_cli_a_{uuid}` and `\\.\pipe\serial_cli_b_{uuid}`
- Use Windows API `CreateNamedPipeW`
- Pipes can be opened like regular files

**Key responsibilities:**
- Pipe creation with Windows API
- Health checking via pipe handle validation
- Automatic cleanup on drop

### Socat Backend (New)

**Platform:** Cross-platform (requires socat binary)

**Implementation approach:**
- Spawn socat process: `socat -d -d pty,raw,echo=0,link=... pty,raw,echo=0,link=...`
- Use symbolic links for stable path names
- Track child process for health and cleanup

**Dependency handling:**
- Assume pre-installed (user: `apt install socat` or `brew install socat`)
- Show helpful error message if socat not found
- Fail fast with clear error on initialization

**Key responsibilities:**
- Process spawning and monitoring
- Health checking via process ID
- Graceful termination (SIGTERM, fallback to SIGKILL)

---

## Error Handling

### New Error Variants

```rust
#[derive(Debug, thiserror::Error)]
pub enum SerialError {
    #[error("Unsupported backend for this platform: {0}")]
    UnsupportedBackend(String),
    
    #[error("Missing required dependency: {0}\nHint: {1}")]
    MissingDependency(String, String),
    
    #[error("Backend initialization failed: {0}")]
    BackendInitFailed(String),
}
```

### Failure Strategy

**Fail fast** - If selected backend fails to initialize, error immediately with:
- Clear error message
- Platform-specific hints
- Suggestion to try alternative backends

**No automatic fallback** - Predictable behavior, user explicitly selects backend

---

## Testing Strategy

### Unit Tests

1. **BackendType parsing** - Verify string → enum conversion
2. **Platform detection** - Verify auto-detection returns correct type per platform
3. **Factory priority chain** - Test CLI flag → config → auto-detect order
4. **Validation logic** - Test missing dependency detection

### Integration Tests

1. **PTY backend lifecycle** - Create, health check, stats, cleanup (Unix only)
2. **NamedPipe backend** - Full lifecycle test (Windows only)
3. **Socat backend** - Full lifecycle test when socat available

### Test Coverage Targets

- Backend trait implementation: 100%
- Factory selection logic: 100%
- Error paths: 80%+

---

## Migration Plan

1. **Create new module structure**
   - `src/serial_core/backends/` directory
   - `trait.rs`, `mod.rs`, `factory.rs`

2. **Extract PTY backend**
   - Refactor existing PTY code into `PtyBackend`
   - Implement `VirtualBackend` trait

3. **Implement new backends**
   - `NamedPipeBackend` for Windows
   - `SocatBackend` for cross-platform

4. **Update VirtualSerialPair**
   - Change to use `Box<dyn VirtualBackend>`
   - Integrate `BackendFactory`

5. **Wire CLI integration**
   - Add `--backend` flag to `virtual create`
   - Update command handler

6. **Update config schema**
   - Add `virtual.default_backend` setting
   - Update ConfigManager

7. **Add tests**
   - Unit tests for each backend
   - Integration tests for factory

8. **Documentation updates**
   - README with backend options
   - Platform-specific installation instructions

---

## Success Criteria

- [ ] All three backends (PTY, NamedPipe, Socat) implement `VirtualBackend` trait
- [ ] BackendFactory correctly implements priority chain (CLI → config → auto)
- [ ] Platform auto-detection works correctly (Windows→NamedPipe, Unix→PTY)
- [ ] Missing dependency errors show helpful hints
- [ ] All unit and integration tests pass
- [ ] Documentation updated with backend usage examples

---

## Open Questions

None - design is complete and approved.

---

## References

- Existing PTY implementation: `src/serial_core/virtual_port.rs`
- Windows Named Pipes API: Microsoft docs
- Socat manual: `man socat`
