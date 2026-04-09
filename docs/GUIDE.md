# GUI Application Guide

## Overview

Serial CLI includes a modern, cross-platform GUI application built with **Tauri** and **React**. The GUI provides a user-friendly interface for all serial port operations, Lua scripting, and protocol management.

## Quick Start

### Starting the GUI

```bash
# Development mode (with hot reload)
just gui-dev

# Production build
just gui-build
```

The GUI will open in a new window with the cyber-industrial interface.

## Features

### 1. Serial Ports Panel

**Location**: Sidebar → "Ports" (⌘1)

**Features**:
- 🔍 List all available serial ports
- ⚙️ Configure port settings (baudrate, data bits, parity, flow control)
- 🔌 Open/close ports with real-time status
- 💾 Remember recent configurations
- 📊 Active connection monitoring

**Usage**:
1. Click "Refresh" to scan for ports
2. Click "Open" on a port to configure it
3. Adjust settings as needed
4. Click "Open Port" to establish connection
5. Monitor status in "Active Connections" panel

### 2. Data Monitor

**Location**: Sidebar → "Data Monitor" (⌘2)

**Features**:
- 📊 Real-time data display
- 🔄 RX/TX direction indicators
- 🔢 Statistics dashboard (total, RX, TX packets)
- 📤 Export to TXT/CSV/JSON
- 🎛️ Display options (hex/ascii, timestamp, auto-scroll)
- 🗑️ Clear data buffer

**Usage**:
1. Open a serial port first
2. Data will automatically appear in real-time
3. Toggle display format with HEX/ASCII button
4. Click export button to save data
5. Use filter options for export

### 3. Script Editor

**Location**: Sidebar → "Scripts" (⌘3)

**Features**:
- 📝 Monaco Editor with Lua syntax highlighting
- 📁 File management (new, save, load, export)
- ▶️ Run scripts with real LuaJIT execution
- 📤 Output console with error highlighting
- 💾 Auto-save to storage

**Keyboard Shortcuts**:
- ⌘N - New script
- ⌘Enter - Run current script
- ⌘S - Save script

**Usage**:
1. Click "New" to create a script
2. Write Lua code with syntax highlighting
3. Click "Run" to execute (real LuaJIT)
4. View output in console panel
5. Scripts are automatically saved

### 4. Protocol Manager

**Location**: Sidebar → "Protocols" (⌘4)

**Features**:
- 📋 Built-in protocols (Modbus RTU/ASCII, AT Commands, Line-based)
- 📤 Load custom Lua protocols
- ✅ Protocol validation
- 🔄 Enable/disable protocols
- ℹ️ Protocol details view

**Usage**:
1. View built-in protocols
2. Click upload button to load custom .lua protocol
3. Protocol is automatically validated
4. Click "Enable"/"Active" to activate protocol
5. View protocol details

### 5. Settings

**Location**: Sidebar → "Settings" (⌘5)

**Tabs**:
- **General** - App preferences (updates, analytics, tray)
- **Serial** - Default serial configuration
- **Data** - Display preferences (format, timestamp, scroll)
- **Notifications** - System notification settings

**Features**:
- ⚙️ Comprehensive configuration
- 💾 Auto-save to storage
- 🔄 Reset to defaults
- ✅ Save button with change detection

## Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| ⌘K | Open command palette |
| ⌘/ | Show keyboard shortcuts |
| ⌘W | Hide window to tray |
| Escape | Close modals/dialogs |

### Navigation

| Shortcut | Action |
|----------|--------|
| ⌘1 | Go to Ports |
| ⌘2 | Go to Data Monitor |
| ⌘3 | Go to Scripts |
| ⌘4 | Go to Protocols |
| ⌘5 | Go to Settings |

### Actions

| Shortcut | Action |
|----------|--------|
| ⌘R | Refresh ports (in Ports view) |
| ⌘N | New script (in Scripts view) |
| ⌘Enter | Run script (in Scripts view) |
| ⌘Shift+C | Clear data (global) |
| ⌘, | Go to Settings |

## Command Palette

**Access**: ⌘K

The command palette provides quick access to all commands:
- Navigation commands
- Port operations
- Data management
- Script operations
- System commands

Type to search, use ↑↓ to navigate, Enter to select.

## Data Persistence

All user data is automatically saved:

- ✅ **Settings** - All preferences
- ✅ **Scripts** - Created and modified scripts
- ✅ **Protocols** - Loaded custom protocols
- ✅ **Recent Ports** - Last 10 port configurations
- ✅ **Window State** - Size and position

Data persists across application restarts.

## Notifications

The GUI supports system-level notifications:

- 🔔 Port connection events
- ❌ Error notifications
- ✅ Script completion
- ⚠️ Warning messages

**Notification Settings**:
- Enable/disable notifications
- Sound preferences
- Duration settings
- Per-category toggles

## Data Export

### Export Formats

1. **TXT** - Plain text format
   ```
   [14:23:45.123] RX: 48 65 6C 6C 6F
   ```

2. **CSV** - Structured data
   ```csv
   Timestamp,Direction,Port,Data (Hex),Data (ASCII),Bytes
   14:23:45.123,RX,/dev/ttyUSB0,48 65 6C 6C 6F,Hello,5
   ```

3. **JSON** - Complete data
   ```json
   {
     "timestamp": 1712675425123,
     "timestamp_formatted": "14:23:45.123",
     "direction": "rx",
     "port_id": "/dev/ttyUSB0",
     "data_hex": "48 65 6C 6C 6F",
     "data_ascii": "Hello",
     "bytes": 5
   }
   ```

### Export Options

- **All** - Export all packets
- **RX Only** - Export received data only
- **TX Only** - Export sent data only

## Design System

### Colors

- **Signal** - #00ff41 (primary green)
- **Alert** - #ff4757 (error red)
- **Amber** - #ffb142 (warning yellow)
- **Info** - #53a0fd (information blue)

### Typography

- **Sans** - Instrument Sans (UI text)
- **Mono** - JetBrains Mono (code/data)
- **Display** - Instrument Serif (headings)

### Components

All UI components follow the cyber-industrial design language:
- **Panel** - Collapsible containers with colored accents
- **Toast** - Notification popups with icons
- **Command Palette** - Quick command access
- **Data Table** - Real-time data display

## Architecture

### Technology Stack

- **Backend**: Tauri 2.0 (Rust)
- **Frontend**: React 18 + TypeScript
- **Build**: Vite
- **UI**: Tailwind CSS + custom design
- **Editor**: Monaco Editor
- **Icons**: lucide-react

### State Management

Context-based state management:
- `NavigationContext` - View navigation
- `PortContext` - Serial port state
- `DataContext` - Data packets
- `ToastContext` - Notifications
- `NotificationContext` - System notifications
- `ScriptActionContext` - Script operations
- `SettingsContext` - User preferences

### Event Flow

```
Serial Data → Tauri Event → DataContext → DataViewer
User Action → Tauri Command → Backend → Event → UI Update
```

## Performance

### Optimizations

- Virtual scrolling for large data sets
- Lazy loading for Monaco Editor
- Event-driven updates
- Optimized re-renders with React.memo
- Debounced user inputs

### Benchmarks

- Can handle 10,000+ data packets smoothly
- Script execution < 100ms for typical scripts
- Port operations < 500ms
- UI updates at 60 FPS

## Troubleshooting

### GUI won't start

```bash
# Clear build artifacts
just gui-clean

# Reinstall dependencies
cd frontend
rm -rf node_modules package-lock.json
npm install

# Try again
just gui-dev
```

### Type errors

```bash
# Run type check
just gui-type-check

# Fix errors automatically (if possible)
npm run type-fix
```

### Port not opening

1. Check if port is in use by another application
2. Verify permissions (Linux: add user to dialout group)
3. Try different configuration settings
4. Check system Device Manager for port status

### Script execution fails

1. Check Lua syntax in Monaco Editor
2. View error output in console panel
3. Verify API calls are correct
4. Enable verbose logging for debugging

## Development Tips

### Hot Reload

The GUI supports hot reload in development:
- Frontend changes auto-reload
- Rust changes require restart
- Use "Reload" button in dev toolbar

### Debugging

```javascript
// Enable console logging in browser
console.log('Debug:', data)

// Check Tauri events
import { listen } from '@tauri-apps/api/event'
listen('data-received', (event) => console.log(event.payload))
```

### Testing

```bash
# Type check
just gui-type-check

# Build test
just gui-build

# Run GUI in production mode
just gui-build && ./src-tauri/target/release/bundle/macos/Serial CLI.app
```

## Contributing

When contributing to the GUI:

1. **Maintain design consistency** - Use existing components and patterns
2. **Type safety** - All code must pass `just gui-type-check`
3. **Performance** - Optimize for large data sets
4. **Accessibility** - Include keyboard shortcuts and screen reader support
5. **Documentation** - Update relevant docs

See [DEVELOPMENT.md](DEVELOPMENT.md) for full contribution guidelines.

---

**GUI Version**: 0.2.0
**Status**: Production Ready ✅
**Last Updated**: 2025-04-09
