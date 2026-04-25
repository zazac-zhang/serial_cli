# Sniff — Serial Port Traffic Monitoring

Monitor serial port traffic in real time using a background daemon process. Session state is persisted to the user's cache directory, allowing you to check statistics or stop the session from a different terminal.

## Subcommands

### `sniff start`

Start monitoring a serial port in the background.

```
serial-cli sniff start -p <port> [OPTIONS]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-p`, `--port` | Port name (e.g. `/dev/ttyUSB0`, `COM1`) | *(required)* |
| `-o`, `--output` | File path to save captured packets | auto-generated |
| `-m`, `--max-packets` | Maximum packets to capture (`0` = unlimited) | `0` |
| `--display` | Enable real-time display | `true` |
| `--format` | Display format: `raw` or `hex` | `raw` |

**Examples:**

```bash
# Start monitoring with defaults (raw format, unlimited packets)
serial-cli sniff start -p /dev/ttyUSB0

# Monitor and save captures to a file
serial-cli sniff start -p /dev/ttyUSB0 -o captures.log

# Monitor in hex display mode with a packet limit
serial-cli sniff start -p /dev/ttyUSB0 --format hex -m 1000

# Monitor with output file, hex format, and a limit of 500 packets
serial-cli sniff start -p /dev/ttyUSB0 -o captures.log --format hex -m 500
```

Only one sniff session can be active at a time. If a session is already running, the command will report the active port and PID. Stale sessions (where the daemon process has exited) are automatically cleaned up.

### `sniff stats`

Display statistics for the active sniff session.

```
serial-cli sniff stats
```

Shows:

- **Port** — monitored serial port
- **PID** — background daemon process ID
- **Started** — elapsed time since the session began
- **Max packets** — capture limit (`0` = unlimited)
- **Hex display** — whether hex mode is enabled
- **Output file** — path and line count (if an output file is configured and has been written)

### `sniff stop`

Stop the active sniff session by terminating the background daemon process (SIGTERM, with SIGKILL fallback).

```
serial-cli sniff stop
```

### `sniff save`

Save captured packets from the active session to a specified file path.

```
serial-cli sniff save -p <path>
```

| Flag | Description | Default |
|------|-------------|---------|
| `-p`, `--path` | Destination file path | *(required)* |

Requires that the session was started with an `--output` file. If no output file was configured, the command will fail with an error message.

## Example Workflow

A typical sniffing session:

```bash
# 1. Start monitoring a port, saving captures to a file
serial-cli sniff start -p /dev/ttyUSB0 -o captures.log

# Output:
# ✓ Sniffing started on port: /dev/ttyUSB0 (PID: 12345)
#   Output file: captures.log
#   Max packets: unlimited
#   Use 'sniff stats' to view statistics
#   Use 'sniff stop' to stop sniffing

# 2. Check statistics while the session is running
serial-cli sniff stats

# Output:
# Sniff session statistics:
#   Port:         /dev/ttyUSB0
#   PID:          12345
#   Started:      2m 15s ago
#   Max packets:  unlimited
#   Hex display:  false
#   Output file:  captures.log (847 lines)

# 3. Stop the sniffing session
serial-cli sniff stop

# 4. Save captured packets to a final destination
serial-cli sniff save -p final_capture.txt

# Output:
# ✓ Captured packets saved to: final_capture.txt
```
