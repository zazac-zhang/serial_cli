# Protocol Management Commands

Manage built-in and custom Lua-based protocol definitions.

## Overview

Serial CLI includes several built-in protocols for common serial communication scenarios. Custom protocols can be added via Lua scripts that implement the `Protocol` trait (`parse()`, `encode()`, `reset()`).

```bash
serial-cli protocol <subcommand> [options]
```

## Built-in Protocols

| Name | Description |
|---|---|
| `modbus_rtu` | Modbus RTU protocol (Industrial communication) |
| `modbus_ascii` | Modbus ASCII protocol (Industrial communication) |
| `at_command` | AT Command protocol (Modem control) |
| `line` | Line-based protocol (Text-based communication) |

Built-in protocols are compiled into the binary and cannot be unloaded.

## Subcommands

### `protocol list [--detailed]`

List all registered protocols.

```bash
# Compact listing (names only)
serial-cli protocol list

# Detailed listing with descriptions and file paths
serial-cli protocol list --detailed
```

The `--detailed` flag shows descriptions for built-in protocols and script paths plus load timestamps for custom protocols.

### `protocol info <name>`

Show detailed information about a specific protocol.

```bash
serial-cli protocol info modbus_rtu
serial-cli protocol info my_custom_protocol
```

Output includes the protocol type (built-in or custom), description or script path, version, and load time.

### `protocol validate <path>`

Validate a Lua protocol script without loading or registering it. Useful for checking scripts before deployment.

```bash
serial-cli protocol validate /path/to/my_protocol.lua
```

The validator checks that the script implements the required `Protocol` trait methods and contains no Lua syntax errors.

### `protocol load <path> [--name <name>]`

Validate and register a custom protocol from a Lua script file.

```bash
# Name inferred from filename (without extension)
serial-cli protocol load /path/to/my_protocol.lua

# Explicit name override
serial-cli protocol load /path/to/my_protocol.lua --name my_protocol
```

The script is validated first, then saved to the configuration file so it persists across sessions. Built-in protocol names are reserved and cannot be reused.

### `protocol unload <name>`

Remove a custom protocol from the configuration.

```bash
serial-cli protocol unload my_custom_protocol
```

Built-in protocols cannot be unloaded. Attempting to unload a built-in returns an error.

### `protocol reload <name>`

Re-validate and reload a custom protocol from its original script path.

```bash
serial-cli protocol reload my_custom_protocol
```

Useful after editing a protocol script to pick up changes without restarting the application. The script path is the one recorded at load time.

### `protocol hot-reload <action>`

Manage automatic hot-reloading of custom protocol scripts.

```bash
serial-cli protocol hot-reload enable
serial-cli protocol hot-reload disable
serial-cli protocol hot-reload status
```

When enabled, the application monitors loaded custom protocol scripts for file changes and automatically reloads them. This is persisted in the configuration file.

## Custom Lua Protocols

Custom protocols are Lua scripts that implement the following interface:

```lua
-- Required methods
function Protocol:parse(data)
    -- Parse incoming raw bytes, return payload or nil on incomplete frame
end

function Protocol:encode(data)
    -- Encode outgoing data into a protocol frame (add headers, checksums, etc.)
end

function Protocol:reset()
    -- Reset internal parser state (optional, no-op by default)
end
```

The engine calls `parse()` on incoming serial data and `encode()` on outgoing data. Scripts are validated before loading to ensure all required methods are present.

See the protocol Lua API reference for the full runtime interface available to custom protocol scripts.

## Full Lifecycle Example

The following sequence demonstrates the complete custom protocol workflow:

```bash
# 1. Validate the script before loading
serial-cli protocol validate /home/user/protocols/custom_proto.lua

# 2. Load the protocol (name inferred from filename stem)
serial-cli protocol load /home/user/protocols/custom_proto.lua

# 3. Verify it appears in the protocol list
serial-cli protocol list --detailed

# 4. View protocol details
serial-cli protocol info custom_proto

# 5. After editing the script, reload to pick up changes
serial-cli protocol reload custom_proto

# 6. When no longer needed, unload it
serial-cli protocol unload custom_proto
```

Alternatively, enable hot-reload so edits are picked up automatically:

```bash
serial-cli protocol hot-reload enable
# Edit custom_proto.lua -- changes are automatically detected and reloaded
serial-cli protocol hot-reload status   # Verify hot-reload is active
```
