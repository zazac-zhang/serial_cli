# Lua Protocol Extension - Design Document

**Date:** 2026-04-05
**Status:** Design Phase
**Author:** AI Assistant

## Executive Summary

This document describes the design for extending the Serial CLI's protocol system to support user-defined Lua protocols. The implementation enables users to load custom protocols from Lua script files, manage them through CLI commands and Lua APIs, and supports automatic hot-reloading when scripts are modified.

## Current State Analysis

### What's Already Implemented

- ✅ `LuaProtocol` core structure (`src/protocol/lua_ext.rs`) - 293 lines
  - Supports `on_frame`, `on_encode`, `on_reset` callbacks
  - Complete unit test coverage (9 test cases)
  - Error handling with passthrough fallback

- ✅ Lua API for using protocols (`src/lua/bindings.rs`)
  - `protocol_encode(name, data)` - Encode data with protocol
  - `protocol_decode(name, data)` - Decode data with protocol
  - `protocol_list()` - List all protocols
  - `protocol_info(name)` - Get protocol information

- ✅ Example scripts
  - `examples/custom_protocol.lua` - Complete custom protocol example (220 lines)

### What's Missing

- ❌ No API to dynamically register custom protocols from Lua
- ❌ No protocol hot-reload mechanism
- ❌ No CLI commands to load/manage Lua protocols
- ❌ `LuaProtocol` not exported from `protocol/mod.rs`
- ❌ No runtime integration (config loading, file watching)

## Design Goals

1. **File-based Loading** - Load protocols from standalone `.lua` files
2. **Persistent Storage** - Save protocol references to config file for auto-loading on startup
3. **Full CRUD Operations** - load, unload, reload, list, info
4. **Original Path Reference** - Store original file paths, not copies
5. **Auto Hot-reload** - Monitor file changes and automatically reload protocols
6. **Tiered Validation** - Strict syntax validation at load time, graceful degradation at runtime

## Architecture

### Module Structure

```
src/protocol/
├── manager.rs          # NEW - ProtocolManager core logic
├── loader.rs           # NEW - Protocol script loader
├── validator.rs        # NEW - Lua script validator
├── watcher.rs          # NEW - File watcher wrapper around notify crate
└── mod.rs              # MODIFY - Export new modules, LuaProtocol

src/lua/bindings.rs     # MODIFY - Add protocol_load/unload/reload API
src/cli/commands.rs     # MODIFY - Add protocol subcommands
src/config.rs           # MODIFY - Add protocols config section
```

### Core Components

#### ProtocolManager (New Module)

```rust
pub struct ProtocolManager {
    registry: ProtocolRegistry,                       // Runtime protocol registry
    config: Config,                                   // Config access
    watcher: Option<notify::RecommendedWatcher>,      // File watcher (notify crate)
    custom_protocols: HashMap<String, CustomProtocol>, // Custom protocol metadata
}

struct CustomProtocol {
    name: String,
    script_path: PathBuf,     // Original file path (reference)
    loaded_at: SystemTime,    // Load timestamp
    version: u64,             // Version number (for hot-reload detection)
}
```

**Responsibilities:**
- Load/unload/reload/list custom protocols
- Persist protocol configuration to file
- Monitor protocol files for changes
- Validate protocol script syntax
- Create protocol instances via ProtocolRegistry

### Data Flow

#### CLI Command Flow

```bash
serial-cli protocol load /path/to/my_protocol.lua

CLI Commands
  → ProtocolManager::load_protocol(path)
    → Validator::validate_syntax(path)      # Syntax validation
    → Config::add_protocol(name, path)      # Save to config
    → ProtocolRegistry::register(factory)   # Register to runtime
    → FileWatcher::watch(path)              # Start monitoring
  → Return success/failure message
```

#### Lua API Flow

```lua
protocol_load("/path/to/custom.lua")
protocol_encode("custom", "data")

Lua script
  → LuaBindings::protocol_load(path)
    → ProtocolManager::load_protocol(path)
  → LuaBindings::protocol_encode(name, data)
    → ProtocolManager::get_protocol(name)  # Get from ProtocolRegistry
    → Protocol::encode(data)
```

#### Hot-reload Flow

```
File System: my_protocol.lua modified
  ↓
FileWatcher detects change
  ↓
ProtocolManager::reload_protocol(name)
  → Validator::validate_syntax(path)       # Validate new version
  → ProtocolRegistry::register(factory)    # Re-register
  → Update version number
  → Log: [INFO] Protocol 'my_protocol' reloaded
```

### Configuration File Format

```toml
# ~/.config/serial-cli/config.toml

[protocols.custom.my_protocol]
name = "my_protocol"
path = "/home/user/protocols/my_protocol.lua"
version = 3
loaded_at = "2026-04-05T10:30:00Z"

[protocols.custom.sensor_protocol]
name = "sensor_protocol"
path = "/home/user/sensor.lua"
version = 1
loaded_at = "2026-04-05T09:15:00Z"
```

## API Design

### CLI Commands

```bash
# Load protocol
serial-cli protocol load <path>
  [ --name <name> ]    # Specify protocol name (default: from filename)
  [ --no-watch ]       # Disable auto monitoring

# Unload protocol
serial-cli protocol unload <name>

# Reload protocol
serial-cli protocol reload <name>

# List all protocols
serial-cli protocol list
  [ --verbose ]  # Show detailed info (path, version, load time)

# Get protocol info
serial-cli protocol info <name>

# Validate protocol script (without loading)
serial-cli protocol validate <path>
```

### Lua API

```lua
-- Load protocol
local ok, err = protocol_load(path)
-- ok: boolean, err: string

-- Unload protocol
local ok, err = protocol_unload(name)

-- Reload protocol
local ok, err = protocol_reload(name)

-- List all protocols (built-in + custom)
local protocols = protocol_list()
-- Returns: { {name="modbus_rtu", type="builtin"},
--           {name="my_proto", type="custom", path="..."} }

-- Get protocol info
local info = protocol_info(name)
-- Returns: { name="my_proto", path="...", version=3, loaded_at="..." }

-- Validate protocol script
local ok, err = protocol_validate(path)
```

### Rust Internal API

```rust
impl ProtocolManager {
    // Load protocol
    pub async fn load_protocol(&mut self, path: &Path) -> Result<ProtocolInfo>;

    // Unload protocol
    pub async fn unload_protocol(&mut self, name: &str) -> Result<()>;

    // Reload protocol
    pub async fn reload_protocol(&mut self, name: &str) -> Result<()>;

    // List all protocols
    pub async fn list_protocols(&self) -> Vec<ProtocolInfo>;

    // Get protocol info
    pub async fn get_protocol_info(&self, name: &str) -> Result<CustomProtocol>;

    // Validate protocol script
    pub fn validate_protocol(path: &Path) -> Result<()>;

    // Start file watcher
    pub fn start_watcher(&mut self) -> Result<()>;
}
```

## Error Handling

### Tiered Validation Strategy

#### 1. Load-time Validation (Strict)

```rust
pub fn validate_protocol(path: &Path) -> Result<()> {
    // Check file exists
    if !path.exists() {
        return Err(SerialError::ProtocolNotFound(path.to_path_buf()));
    }

    // Check file readable
    let script = fs::read_to_string(path)?;

    // Validate Lua syntax
    let lua = Lua::new();
    lua.load(&script)
        .exec()
        .map_err(|e| SerialError::InvalidScript {
            path: path.clone(),
            error: e.to_string(),
        })?;

    // Validate required functions exist
    lua.load("
        if type(on_frame) ~= 'function' then
            error('Missing required function: on_frame')
        end
        if type(on_encode) ~= 'function' then
            error('Missing required function: on_encode')
        end
    ").exec()?;

    Ok(())
}
```

**Error Example:**
```bash
$ serial-cli protocol load bad_protocol.lua
[ERROR] Cannot load protocol: Syntax error at line 15:
       unexpected token near 'end'
```

#### 2. Runtime Error (Graceful Degradation)

```rust
// In LuaProtocol
fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>> {
    match self.execute_callback("on_frame", data) {
        Ok(result) => {
            self.stats.errors = 0;  // Reset error count
            Ok(result)
        }
        Err(e) => {
            self.stats.errors += 1;

            // Fallback to passthrough mode
            log::warn!("Protocol callback failed, using passthrough: {}", e);
            Ok(data.to_vec())
        }
    }
}
```

**Runtime Error Example:**
```
[WARN] Protocol 'my_protocol' callback failed:
       attempt to index local 'data' (a nil value)
       Using passthrough mode (errors: 1)
```

#### 3. Error Recovery Strategy

- Consecutive errors > 10: Auto-disable protocol and notify
- Single error: Log warning, degrade to passthrough
- File watcher detects fix: Auto-reload and clear error count

### Configuration Error Handling

```rust
// Load config on startup
if let Err(e) = manager.load_all_from_config().await {
    log::error!("Failed to load protocols from config: {}", e);
    // Continue startup, don't block built-in protocols
}

// Single protocol failure doesn't affect others
for (name, proto_config) in config.protocols {
    match manager.load_protocol(&proto_config.path).await {
        Ok(_) => log::info!("Loaded protocol: {}", name),
        Err(e) => log::warn!("Skipped protocol '{}': {}", name, e),
    }
}
```

## Testing Strategy

### Unit Tests

**Protocol Validator** (`protocol/validator.rs`)
```rust
#[test]
fn test_valid_protocol() { /* ... */ }
#[test]
fn test_missing_function() { /* ... */ }
#[test]
fn test_syntax_error() { /* ... */ }
```

**Protocol Manager** (`protocol/manager.rs`)
```rust
#[tokio::test]
async fn test_load_and_list_protocol() { /* ... */ }
#[tokio::test]
async fn test_unload_protocol() { /* ... */ }
#[tokio::test]
async fn test_reload_protocol() { /* ... */ }
```

### Integration Tests

**Lua API Integration** (`tests/lua_protocol_manager_test.rs`)
```rust
#[tokio::test]
async fn test_lua_protocol_load_workflow() {
    // Test complete load → list → use workflow
}
```

### End-to-End Tests

**CLI Commands** (`tests/cli_protocol_e2e_test.rs`)
- Test complete workflow: create → load → list → use → modify → reload → unload

### Test Coverage Targets

- **Unit Tests**: 90%+ core logic coverage
- **Integration Tests**: Cover all Lua APIs
- **E2E Tests**: Cover major CLI command flows

## Implementation Phases

### Phase 1: Core Infrastructure (Priority: High)
- Create `ProtocolManager` structure
- Implement `loader.rs` for loading Lua scripts
- Implement `validator.rs` for syntax validation
- Update `protocol/mod.rs` to export `LuaProtocol`

### Phase 2: Configuration Persistence (Priority: High)
- Extend `config.rs` to support `[protocols.custom]` section
- Implement load/save of protocol configuration
- Auto-load protocols on startup

### Phase 3: Lua API Integration (Priority: High)
- Add `protocol_load/unload/reload` to `lua/bindings.rs`
- Integrate with ProtocolManager
- Add integration tests

### Phase 4: CLI Commands (Priority: Medium)
- Add `protocol` subcommand to `cli/commands.rs`
- Implement load/unload/reload/list/info/validate commands
- Add CLI tests

### Phase 5: File Watching (Priority: Medium)
- Implement `watcher.rs` as wrapper around `notify` crate
- Use `notify::RecommendedWatcher` for cross-platform support
- Auto-reload on file changes
- Handle file deletion/recreation gracefully
- Log watcher errors, provide manual reload fallback

### Phase 6: Error Handling & Polish (Priority: Low)
- Implement error recovery strategies
- Add detailed error messages
- Performance optimization
- Documentation updates

## Dependencies

### New Dependencies

```toml
[dependencies]
notify = "6.0"  # File watching
```

## Risks and Mitigations

### Risk 1: File Watching Complexity
- **Risk**: File watching can be unreliable on some platforms
- **Mitigation**: Provide manual reload command as fallback, log watcher errors

### Risk 2: Lua Script Errors
- **Risk**: Malicious or buggy scripts could crash the system
- **Mitigation**: Strict validation at load time, isolated Lua instances, graceful degradation

### Risk 3: Configuration Corruption
- **Risk**: Config file could become corrupted
- **Mitigation**: Validate config on load, provide recovery commands, backup config

## Future Enhancements

- Protocol versioning and migration
- Remote protocol repository
- Protocol signature verification
- Protocol sandboxing
- Protocol dependency management

## Acceptance Criteria

- [ ] Users can load Lua protocols from files
- [ ] Protocols are persisted across restarts
- [ ] File changes trigger automatic reload
- [ ] CLI and Lua APIs work correctly
- [ ] Errors are handled gracefully
- [ ] All tests pass (90%+ coverage)
- [ ] Documentation is updated

## References

- Existing `src/protocol/lua_ext.rs` implementation
- `examples/custom_protocol.lua` example
- `TODO.md` line 86: "Lua 自定义协议加载器"
- Serial CLI DEVELOPMENT.md

---

**Document Version:** 1.0
**Last Updated:** 2026-04-05
