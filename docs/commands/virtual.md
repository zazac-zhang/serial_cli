# Virtual Serial Port Commands

Manage virtual serial port pairs that bridge data between two pseudo-terminals or named pipes.
Useful for testing, protocol development, and debugging without physical hardware.

## Overview

The `virtual` command group creates bidirectional bridges between two virtual port endpoints.
Data written to one port is forwarded to the other, mimicking a null-modem cable.
An in-memory registry tracks all active pairs for the lifetime of the CLI process.

```
serial-cli virtual <subcommand>
```

## Subcommands

### `virtual create`

Create a new virtual serial port pair.

```
serial-cli virtual create [OPTIONS]
```

| Option | Description |
|---|---|
| `--backend <type>` | Backend type: `auto` (default), `pty`, `socat`, `namedpipe` |
| `--monitor` | Enable traffic monitoring on the bridge |
| `-o, --output <file>` | Save monitored traffic to a file |
| `--max-packets <n>` | Maximum packets to capture (0 = unlimited, default: 0) |

On creation, the command prints the pair ID, both port paths, and the backend in use.

### `virtual list`

List all active virtual port pairs.

```
serial-cli virtual list
```

Displays for each pair: ID, port A/B names, backend, uptime, status, bytes bridged,
packets bridged, and bridge error count.

### `virtual stop <id>`

Stop and tear down a virtual port pair.

```
serial-cli virtual stop <id>
```

The pair is removed from the registry and its underlying OS resources are released.

### `virtual stats <id>`

Show detailed statistics for a specific virtual port pair.

```
serial-cli virtual stats <id>
```

Displays: ID, port A/B names, backend, running status, uptime, bytes bridged,
packets bridged, bridge error count, and last error (if any).

## Backend Comparison

| Backend | Platform | Requirements | Notes |
|---|---|---|---|
| `pty` | Unix / macOS | None | POSIX pseudo-terminals. Native, no external dependencies. |
| `namedpipe` | Windows | None | Windows named pipes (`\\.\pipe\...`). Native to the OS. |
| `socat` | Any | `socat` binary on PATH | Spawns `socat` as a child process. Cross-platform but requires external tool. |
| `auto` | Any | None | Auto-detects the best native backend for the current platform (`pty` on Unix, `namedpipe` on Windows). |

Default: `auto` -- no flag needed for typical use.

## Example Workflow

```bash
# 1. Create a virtual port pair
$ serial-cli virtual create --backend auto --monitor
  Virtual port pair created
    ID: a1b2c3d4-e5f6-7890-abcd-ef1234567890
    Port A: /dev/pts/3
    Port B: /dev/pts/4
    Backend: Pty
    Monitoring: enabled (max 0 packets)

# 2. List active pairs
$ serial-cli virtual list
  Active virtual port pairs:

    ID: a1b2c3d4-e5f6-7890-abcd-ef1234567890
      Port A: /dev/pts/3
      Port B: /dev/pts/4
      Backend: Pty
      Uptime: 45s
      Status: Running
      Bytes bridged: 1024
      Packets bridged: 0

# 3. Use the ports with other tools
# In one terminal:
$ cat /dev/pts/3

# In another terminal:
$ echo "hello" > /dev/pts/4

# 4. Check statistics
$ serial-cli virtual stats a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Virtual pair statistics:
    ID: a1b2c3d4-e5f6-7890-abcd-ef1234567890
    Port A: /dev/pts/3
    Port B: /dev/pts/4
    Backend: Pty
    Status: Running
    Uptime: 120s
    Bytes bridged: 2048
    Packets bridged: 0
    Bridge errors: 0

# 5. Stop the pair when done
$ serial-cli virtual stop a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Virtual pair stopped
```

## Notes

- Pairs are tracked **in memory** only. Restarting the CLI discards the registry.
- The `--monitor` flag enables a sniffer on the bridge that records traffic; use
  `-o` to persist captured packets to a file.
- On Windows, port paths use the named pipe namespace (`\\.\pipe\...`).
- The `socat` backend requires the `socat` binary to be installed and available
  on the system `PATH`.
