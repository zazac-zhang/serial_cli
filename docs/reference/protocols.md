# Protocol Reference

## What Is a Protocol?

In Serial CLI, a **protocol** is an abstraction that defines how raw bytes on a serial line are framed, encoded, and parsed. All protocols -- whether built-in or user-defined in Lua -- implement the `Protocol` trait:

```rust
pub trait Protocol: Send + Sync {
    fn name(&self) -> &str;
    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>>;
    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>>;
    fn reset(&mut self) -> Result<()> { Ok(()) }
    fn has_data(&self) -> bool { false }
    fn stats(&self) -> ProtocolStats { ProtocolStats::default() }
}
```

### Core Methods

| Method    | Direction   | Purpose |
|-----------|-------------|---------|
| `parse`   | Incoming    | Transform raw bytes into a protocol frame. Returns the parsed payload or an error if the frame is malformed, a checksum fails, or data is incomplete. |
| `encode`  | Outgoing    | Frame outgoing data (add headers, checksums, terminators, etc.). |
| `reset`   | --          | Reset internal parser state. Default is a no-op; Lua protocols may define an `on_reset` callback. |

### Supporting Methods

| Method     | Purpose |
|------------|---------|
| `name`     | Returns the protocol's unique identifier (e.g., `"modbus_rtu"`). |
| `has_data` | Indicates whether the protocol has a complete frame ready for consumption. Default returns `false`. |
| `stats`    | Returns cumulative `ProtocolStats` (`frames_parsed`, `frames_encoded`, `errors`). |

---

## Built-In Protocols

Serial CLI ships with four built-in protocols, registered at startup via `register_all_built_in()`:

| Name           | Description |
|----------------|-------------|
| `modbus_rtu`   | Modbus RTU protocol (Binary industrial communication) |
| `modbus_ascii` | Modbus ASCII protocol (Text-based industrial communication) |
| `at_command`   | AT Command protocol (Modem control commands) |
| `line`         | Line-based protocol (Simple text line communication) |

Built-in protocol names are reserved; custom Lua scripts cannot use these names.

---

### modbus_rtu

**Modbus RTU** is the binary variant of the Modbus protocol, widely used in industrial automation and PLC communication.

#### Frame Format

```
[Slave ID][Function Code][Data...][CRC Low][CRC High]
   1 byte      1 byte       N bytes     1 byte      1 byte
```

#### Encoding

- Appends a **CRC-16** checksum (Modbus polynomial `0xA001`, initial value `0xFFFF`) to the payload.
- CRC is stored in **little-endian** order (low byte first).

#### Parsing

- Validates minimum frame length (4 bytes: slave + function + 2-byte CRC).
- Computes CRC over all bytes except the trailing two, compares against received CRC.
- On success, returns the payload with the CRC stripped.
- On mismatch, returns `ChecksumFailed` with expected/got hex values.

#### Use Cases

- Reading/writing registers on Modbus RTU devices (PLCs, sensors, motor drives).
- Environments with reliable serial links where binary efficiency matters.

---

### modbus_ascii

**Modbus ASCII** is the text-encoded variant of Modbus, using ASCII hex representation for human-readable frames.

#### Frame Format

```
: [Slave ID][Function Code][Data...][LRC] \r \n
  2 chars      2 chars       N*2 chars   2 chars    2 chars
```

- Starts with a colon (`:`) delimiter.
- Every byte is represented as two uppercase hex characters.
- Ends with a **Longitudinal Redundancy Check (LRC)** followed by `\r\n`.

#### Encoding

1. Prefix with `:`.
2. Convert each payload byte to two hex characters (`0`-`F`).
3. Compute LRC: `LRC = (~sum(data) + 1) & 0xFF` (two's complement of the byte sum).
4. Append LRC as two hex characters.
5. Suffix with `\r\n`.

#### Parsing

1. Validates the leading `:` delimiter.
2. Locates `\r` end delimiter.
3. Decodes hex pairs into bytes (case-insensitive).
4. Verifies the trailing LRC byte.
5. Returns `(slave_id, function_code, pdu_data)` on success.

#### Use Cases

- Debugging Modbus traffic (frames are human-readable in a terminal/sniffer).
- Devices that only support ASCII mode.
- Scenarios where line-based tools (like `cat` or `minicom`) are used for diagnostics.

---

### at_command

**AT Command** protocol implements the Hayes command set used for modem and cellular module control.

#### Encoding

- Appends a **termination string** (default `\r\n`) to outgoing commands.
- Termination is always appended, even if the input already contains it (unlike the `line` protocol which avoids duplication).

#### Parsing

- Returns data as-is for normal responses.
- Detects `ERROR` substrings in responses and returns a `UnexpectedResponse` error.
- Responses containing `OK` pass through normally.

#### Configuration

| Parameter      | Default | Description |
|----------------|---------|-------------|
| `timeout_ms`   | 1000    | Command timeout in milliseconds |
| `termination`  | `\r\n`  | Command termination sequence |

#### Use Cases

- Configuring cellular modems, GSM/LTE modules (SIM7600, ESP8266, etc.).
- Sending Hayes AT commands to dial-up modems.
- Querying signal strength (`AT+CSQ`), network registration (`AT+CREG?`), etc.

---

### line

**Line** protocol is a simple text-based protocol that treats each line as a message frame.

#### Encoding

- Appends a **separator** (default `\n`) to outgoing data.
- If the data already ends with the separator, it is **not** duplicated.

#### Parsing

- Passes data through unchanged. Frame boundaries are determined by the serial line discipline or external tools (sniffer, timeout).

#### Configuration

| Parameter   | Default | Description |
|-------------|---------|-------------|
| `separator` | `\n`    | Line separator byte(s) |

#### Use Cases

- Interacting with CLI-based serial devices (routers, embedded shells).
- Simple text protocols where each message is a newline-terminated string.
- Prototyping and testing serial communication.

---

## Architecture: The `protocol/` Module

The protocol subsystem is organized into the following components:

```
src/protocol/
  mod.rs          -- Protocol trait definition, ProtocolStats
  built_in/       -- Built-in protocol implementations
    mod.rs        -- BUILTIN_PROTOCOL_NAMES, is_builtin_protocol()
    modbus.rs     -- ModbusProtocol (RTU + ASCII modes)
    at_command.rs -- AtCommandProtocol
    line.rs       -- LineProtocol
  registration.rs -- register_all_built_in()
  registry.rs     -- ProtocolRegistry, ProtocolFactory, ProtocolInfo
  manager.rs      -- ProtocolManager (load/unload/reload lifecycle)
  loader.rs       -- ProtocolLoader, LoadedProtocol
  validator.rs    -- ProtocolValidator, ValidationResult
  watcher.rs      -- ProtocolWatcher (file-based hot-reload)
  lua_ext.rs      -- LuaProtocol, create_lua_protocol()
```

### ProtocolRegistry

The `ProtocolRegistry` stores **factories** (not single instances), enabling multiple consumers to each receive a fresh protocol instance.

```rust
pub trait ProtocolFactory: Send + Sync {
    fn create(&self) -> Result<Box<dyn Protocol>>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
```

Key operations:

| Operation           | Behavior |
|---------------------|----------|
| `register(factory)`  | Registers a factory. Silently replaces existing entries with the same name. |
| `get_protocol(name)` | Creates a fresh protocol instance from the registered factory. |
| `list_protocols()`   | Returns `Vec<ProtocolInfo>` (name + description) for all registered protocols. |
| `unregister(name)`   | Removes a factory by name. |
| `is_registered(name)`| Checks whether a protocol name is registered. |

### ProtocolManager

`ProtocolManager` orchestrates the full lifecycle of custom Lua protocols:

| Method                        | Description |
|-------------------------------|-------------|
| `load_protocol(path)`         | Loads a `.lua` script, validates it, creates a factory, and registers it. |
| `unload_protocol(name)`       | Removes a protocol from both the registry and internal tracking. |
| `reload_protocol(name)`       | Unloads then re-loads from the original file path. |
| `list_protocols()`            | Lists all protocols, marking custom ones with a `(custom)` suffix. |
| `validate_protocol(path)`     | Validates a script without loading or registering it. |
| `enable_hot_reload()`         | Enables automatic file watching and reloading. |
| `disable_hot_reload()`        | Disables hot-reload and aborts the watcher task. |

### ProtocolLoader

`ProtocolLoader` reads Lua scripts from disk and extracts metadata:

1. Validates the script via `ProtocolValidator`.
2. Reads the source content.
3. Extracts the protocol name from a `-- Protocol: <name>` comment, or falls back to the filename stem.
4. Produces a `LoadedProtocol` struct with name, path, content, and timestamp.
5. Creates a `LuaProtocolFactory` that wraps `create_lua_protocol()`.

### ProtocolValidator

Before a Lua script is loaded, the validator performs these checks:

1. **File exists** and is readable.
2. **Lua syntax** is valid (script executes without syntax errors).
3. **Required functions** are present: `on_frame` and `on_encode`.
4. Extracts the protocol name from `-- Protocol: <name>` comment (if present).

### ProtocolWatcher

`ProtocolWatcher` uses the `notify` crate to monitor Lua protocol files:

- Watches the parent directory of loaded `.lua` scripts.
- Filters events to `.lua` file extensions only.
- Emits file paths on `reload_events()` channel for `ProtocolManager` to process.
- Triggers on `Modify`, `Create`, and `Remove` events.

---

## Writing Custom Protocols in Lua

Serial CLI supports defining custom protocols in Lua scripts, loaded at runtime via the `ProtocolManager`.

### Required Functions

Every Lua protocol script must define these two functions:

```lua
-- Parse incoming raw bytes into a protocol frame.
-- Input:  data -- Lua table of bytes (1-indexed)
-- Output: Lua table of bytes (the parsed payload)
function on_frame(data)
    -- ...
    return result
end

-- Encode outgoing data into a protocol frame.
-- Input:  data -- Lua table of bytes (1-indexed)
-- Output: Lua table of bytes (the framed output)
function on_encode(data)
    -- ...
    return result
end
```

### Optional Function

```lua
-- Reset internal parser state (called by Protocol::reset)
function on_reset()
    -- cleanup state
end
```

### Protocol Lifecycle

1. **Validation** -- `ProtocolValidator` checks syntax and required functions.
2. **Loading** -- `ProtocolLoader` reads the script, extracts metadata, creates a factory.
3. **Registration** -- Factory is inserted into `ProtocolRegistry`.
4. **Instantiation** -- Each `get_protocol(name)` call creates a fresh `LuaProtocol` instance via `mlua`.
5. **Execution** -- For each `parse()` or `encode()` call:
   - A fresh Lua VM is created.
   - The script is loaded.
   - Input bytes are passed as a 1-indexed Lua table under the global `data`.
   - The appropriate callback (`on_frame` / `on_encode`) is invoked.
   - The return value is converted back to bytes (supports table, string, integer).
6. **Reset** -- `reset()` optionally calls `on_reset()` if defined.
7. **Hot-reload** -- If enabled, `ProtocolWatcher` detects file changes and triggers `reload_protocol()`.

### Return Value Types

The Lua callback may return:

| Lua Type  | Behavior |
|-----------|----------|
| `table`   | Interpreted as a 1-indexed array of byte values. |
| `string`  | Converted to raw bytes. |
| `number`  | Wrapped as a single-byte result. |
| other     | Falls back to passing through the original input unchanged. |

### Error Handling

If a Lua callback throws an error, the protocol falls back to **passthrough** mode (returns the original input data unchanged). This ensures the communication pipeline remains resilient to script errors.

### Loading and Hot-Reloading

```bash
# Validate a protocol script without loading
serial_cli protocol validate --path my_protocol.lua

# Load a custom protocol
serial_cli protocol load --path my_protocol.lua

# List all registered protocols (built-in + custom)
serial_cli protocol list

# Reload a protocol (manual)
serial_cli protocol reload --name my_protocol

# Unload a protocol
serial_cli protocol unload --name my_protocol
```

Hot-reload monitors loaded `.lua` files for changes. When a file is modified, the protocol is automatically unloaded and re-loaded from disk, bumping the version counter.

---

## Lua Protocol Example

```lua
-- Protocol: custom_binary
-- A simple protocol that wraps data in a length-prefixed frame:
--   [Length: 1 byte][Payload: N bytes][Checksum: 1 byte (XOR of all payload bytes)]

function on_frame(data)
    -- Parse: extract payload from [len][payload][checksum]
    if #data < 3 then
        return data  -- passthrough if too short
    end

    local len = data[1]
    if #data < 2 + len then
        return data  -- incomplete frame
    end

    -- Verify checksum (XOR of payload bytes)
    local checksum = 0
    for i = 2, 1 + len do
        checksum = checksum ~ data[i]
    end

    local expected_checksum = data[2 + len]
    if checksum ~= expected_checksum then
        error("Checksum mismatch")
    end

    -- Return payload only
    local result = {}
    for i = 2, 1 + len do
        table.insert(result, data[i])
    end
    return result
end

function on_encode(data)
    -- Encode: wrap in [len][payload][checksum]
    local len = #data
    local checksum = 0
    for _, byte in ipairs(data) do
        checksum = checksum ~ byte
    end

    local result = {}
    table.insert(result, len)        -- length prefix
    for _, byte in ipairs(data) do
        table.insert(result, byte)   -- payload
    end
    table.insert(result, checksum)   -- checksum
    return result
end

function on_reset()
    -- No persistent state to reset
end
```

---

## Protocol With Lua Extensibility

The Lua integration (`lua_ext.rs`) bridges the `Protocol` trait to Lua scripts through the `LuaProtocol` struct:

- **Thread safety**: Each `parse()`/`encode()` call creates a fresh `mlua::Lua` VM, ensuring `Send + Sync` compatibility.
- **Data marshalling**: Rust byte slices are converted to 1-indexed Lua tables; return values are converted back.
- **Statistics**: `LuaProtocol` tracks `frames_parsed`, `frames_encoded`, and `errors` in `ProtocolStats`.
- **Script caching**: The Lua source is stored within the `LuaProtocol` instance and re-executed on each call.

### Architecture Data Flow

```
User's Lua Script (.lua)
    |
    v
ProtocolValidator (syntax + required functions)
    |
    v
ProtocolLoader (metadata extraction, LoadedProtocol)
    |
    v
ProtocolLoader::create_factory() --> LuaProtocolFactory
    |
    v
ProtocolRegistry::register(factory)
    |
    +---> get_protocol(name) --> factory.create() --> LuaProtocol
                                    |
                                    v
                              parse(data) --> on_frame(data) [Lua VM]
                              encode(data) --> on_encode(data) [Lua VM]
                              reset() --> on_reset() [Lua VM, optional]
    |
    v
ProtocolWatcher (file change events --> ProtocolManager::reload_protocol)
```
