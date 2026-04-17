# Virtual Serial Ports

**Overview**: Virtual serial port pairs enable testing and debugging serial communication without physical hardware.

## Quick Start

```bash
# Create a virtual pair
serial-cli virtual create

# Use in two terminals
# Terminal 1: serial-cli interactive --port /dev/ttys014
# Terminal 2: serial-cli interactive --port /dev/ttys015

# Manage virtual pairs
serial-cli virtual list
serial-cli virtual stats <id>
serial-cli virtual stop <id>
```

## Features

- **Bidirectional Bridging**: Full PTY-to-PTY data forwarding
- **Configuration Support**: Customizable buffer sizes and polling
- **Error Tracking**: Detailed bridge statistics and error reporting
- **Resource Management**: Automatic cleanup, no file descriptor leaks

## Platform Support

| Platform | Backend | Status |
|----------|---------|--------|
| Linux    | PTY     | ✅ Full support |
| macOS    | PTY     | ✅ Full support |
| Windows  | -       | 🚧 Planned |

## Known Limitations

- **Monitoring**: Limited monitoring support (use real ports for full monitoring)
- **Process-Local**: Must manage virtual pairs in the same terminal
- **Polling**: Uses 1ms polling (not true async I/O)

## Configuration

```toml
[virtual]
backend = "pty"              # Backend type
bridge_buffer_size = 8192   # Buffer size
monitor = false              # Enable monitoring
```

## Implementation Details

**Backend**: POSIX PTY (pseudo-terminal)
**Architecture**: Two PTY masters bridged by background task
**Statistics**: Bytes/packets bridged, error count, last error

See [`src/serial_core/virtual_port.rs`](../../src/serial_core/virtual_port.rs) for implementation.
