# Batch Script Execution

**Command group**: `serial-cli batch [subcommand]`

Run multiple Lua scripts sequentially or through a batch file with variable substitution, loops, and sleep delays.

## Subcommands

### `batch run <file>`

Execute a single Lua script or a multi-line batch file.

```
serial-cli batch run <file> [OPTIONS]
```

**File detection**:
- `.lua` extension -- runs as a single Lua script
- Any other extension -- parsed as a batch file (see Batch File Syntax below)

| Option | Default | Description |
|---|---|---|
| `--concurrent <N>` | `5` | Maximum concurrent tasks |
| `--continue-on-error` | `false` | Continue executing after a script fails |
| `--timeout <N>` | `60` | Per-task timeout in seconds |

**Examples**:

```bash
# Run a single Lua script
serial-cli batch run scripts/read_registers.lua

# Run a batch file with 10 concurrent tasks
serial-cli batch run tasks.batch --concurrent 10

# Continue on error with 30s timeout per task
serial-cli batch run tasks.batch --continue-on-error --timeout 30
```

### `batch list`

Search for batch scripts in the current working directory and `~/.config/serial_cli`. Files with extensions `.batch`, `.txt`, or `.lua` are listed.

```bash
serial-cli batch list
```

## Batch File Syntax

A batch file is a plain text file with one directive per line.

| Directive | Description |
|---|---|
| `# comment` | Comment line (ignored during execution) |
| `set NAME value` | Assign a variable |
| `loop N` ... `end` | Repeat the enclosed block N times |
| `sleep MS` | Pause for N milliseconds |
| `<file path>` | Execute the referenced Lua script |

### Variables

Variables set with `set` are substituted into later lines using `${VAR}` or `$VAR` syntax. Unresolved variables fall back to environment variables, then expand to an empty string.

```
set PORT /dev/ttyUSB0
scripts/read_${PORT}.lua    # resolves to scripts/read_/dev/ttyUSB0.lua
```

### Loops

The `loop N` ... `end` block repeats all enclosed directives N times. Nested loops are not supported.

```
loop 3
  scripts/measure.lua
  sleep 1000
end
```

### Sleep

`sleep MS` introduces a delay of the specified number of milliseconds.

## Example Batch File

```batch
# Device test suite - runs diagnostics across multiple ports
set DEVICE /dev/ttyUSB0
set BAUDRATE 115200
set ITERATIONS 5

# Initial handshake
scripts/handshake.lua --port $DEVICE --baud $BAUDRATE

# Run measurement loop 5 times
loop 5
  scripts/read_sensors.lua --port $DEVICE
  sleep 500
  scripts/write_config.lua --port $DEVICE --baud $BAUDRATE
end

# Final validation
scripts/validate_results.lua
```

## Output

On completion, a summary is printed:

```
Batch execution completed:
  Total scripts: 12
  Successful: 11
  Failed: 1
```

When `--continue-on-error` is not set (the default), execution stops at the first failure and remaining scripts are not attempted.
