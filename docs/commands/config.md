# Config Command

> `serial-cli config` -- manage TOML-based configuration for serial ports, logging, Lua, tasks, output, and virtual ports.

## Overview

Serial CLI uses a TOML-based configuration system with three-tier loading:

1. **Project-level**: `.serial-cli.toml` in the current directory
2. **Global-level**: platform-specific config directory
   - **macOS / Linux**: `$XDG_CONFIG_HOME/serial-cli/config.toml` (typically `~/.config/serial-cli/config.toml`)
   - **Windows**: `%APPDATA%\serial-cli\config.toml`
3. **Built-in defaults**: compiled-in fallback values

Changes made with `config set` only affect the in-memory configuration. Use `config save` to persist them to disk.

## Subcommands

### `config show [--json]`

Display the current (in-memory) configuration.

```bash
# Human-readable TOML-style view
serial-cli config show

# Machine-readable JSON
serial-cli config show --json
```

### `config set <key> <value>`

Modify a configuration value in memory. Changes are ephementual -- they last only for the current invocation unless saved with `config save`.

```bash
serial-cli config set serial.baudrate 9600
serial-cli config set logging.level debug
serial-cli config set output.json_pretty false
```

#### Valid Configuration Keys

| Key | Type | Valid Values | Default | Description |
|---|---|---|---|---|
| `serial.baudrate` | `u32` | Any positive integer | `115200` | Serial baud rate |
| `serial.databits` | `u8` | `5`--`8` | `8` | Number of data bits |
| `serial.stopbits` | `u8` | `1`--`2` | `1` | Number of stop bits |
| `serial.parity` | `String` | `none`, `odd`, `even` | `none` | Parity mode |
| `serial.timeout_ms` | `u64` | Any integer | `1000` | Read/write timeout in milliseconds |
| `logging.level` | `String` | `error`, `warn`, `info`, `debug`, `trace` | `info` | Minimum log level |
| `logging.format` | `String` | `text`, `json` | `text` | Log output format |
| `logging.file` | `String` | File path (empty = stdout) | *(empty)* | Log file destination |
| `lua.memory_limit_mb` | `usize` | Any integer | `128` | LuaJIT memory cap in MB |
| `lua.timeout_seconds` | `u64` | Any integer | `300` | Script execution timeout |
| `lua.enable_sandbox` | `bool` | `true`, `false` | `true` | Enable Lua sandbox restrictions |
| `task.max_concurrent` | `usize` | Any positive integer | `10` | Maximum concurrent async tasks |
| `task.default_timeout_seconds` | `u64` | Any integer | `60` | Default task timeout |
| `output.json_pretty` | `bool` | `true`, `false` | `true` | Pretty-print JSON output |
| `output.show_timestamp` | `bool` | `true`, `false` | `true` | Include timestamps in output |
| `virtual.backend` | `String` | `pty`, `socat`, `namedpipe` | `pty` | Virtual port backend |
| `virtual.monitor` | `bool` | `true`, `false` | `false` | Enable traffic monitoring by default |
| `virtual.monitor_format` | `String` | `hex`, `raw` | `hex` | Monitor output format |
| `virtual.auto_cleanup` | `bool` | `true`, `false` | `true` | Clean up virtual ports on exit |
| `virtual.max_packets` | `usize` | Any integer (`0` = unlimited) | `0` | Maximum packets to capture |
| `virtual.bridge_buffer_size` | `usize` | Any integer | `8192` | Bridge buffer size in bytes |
| `virtual.bridge_poll_interval_ms` | `u64` | Any integer | `10` | Bridge polling interval in ms |

### `config save [--path <path>]`

Persist the current in-memory configuration to a TOML file.

```bash
# Save to default location (project .serial-cli.toml or global config)
serial-cli config save

# Save to a specific path
serial-cli config save --path /etc/serial-cli/config.toml
serial-cli config save --path ./my-config.toml
```

Parent directories are created automatically if they do not exist.

### `config reset`

Restore all configuration values to their built-in defaults. Changes are in-memory only; use `config save` to persist.

```bash
serial-cli config reset
```

## Default Configuration

The built-in defaults, expressed as TOML:

```toml
[serial]
baudrate = 115200
databits = 8
stopbits = 1
parity = "none"
timeout_ms = 1000

[logging]
level = "info"
format = "text"
file = ""

[lua]
memory_limit_mb = 128
timeout_seconds = 300
enable_sandbox = true

[task]
max_concurrent = 10
default_timeout_seconds = 60

[output]
json_pretty = true
show_timestamp = true

[virtual]
backend = "pty"
monitor = false
monitor_format = "hex"
auto_cleanup = true
max_packets = 0
bridge_buffer_size = 8192
bridge_poll_interval_ms = 10
```

## Example Workflows

### Configure a legacy RS-232 device at 9600 baud, 7 data bits, even parity, 2 stop bits

```bash
serial-cli config set serial.baudrate 9600
serial-cli config set serial.databits 7
serial-cli config set serial.parity even
serial-cli config set serial.stopbits 2
serial-cli config set serial.timeout_ms 2000
serial-cli config save
```

### Enable debug logging with JSON format for troubleshooting

```bash
serial-cli config set logging.level debug
serial-cli config set logging.format json
serial-cli config set logging.file /tmp/serial-debug.log
serial-cli config save
```

### Disable Lua sandbox for trusted scripts in a controlled environment

```bash
serial-cli config set lua.enable_sandbox false
serial-cli config set lua.memory_limit_mb 256
serial-cli config save
```

### Set up virtual ports with monitoring for protocol debugging

```bash
serial-cli config set virtual.monitor true
serial-cli config set virtual.monitor_format raw
serial-cli config set virtual.max_packets 1000
serial-cli config save
```

### Configure for CI/CD pipeline (minimal output, no timestamps)

```bash
serial-cli config set output.json_pretty false
serial-cli config set output.show_timestamp false
serial-cli config save --path .serial-cli.toml
```

## Validation

The configuration manager validates settings when `config set` is called and warns if the result is invalid. Validation rules:

- **baudrate** must not be zero
- **databits** must be 5--8
- **stopbits** must be 1 or 2
- **parity** must be `none`, `odd`, or `even`
- **logging.level** must be one of `error`, `warn`, `info`, `debug`, `trace`
- **max_concurrent** must not be zero

A warning is printed if validation fails, but the value is still accepted in memory -- invalid configurations cannot be saved to a file (the save operation will reject them).
