# 📚 Documentation Index

Complete documentation for Serial CLI project.

## User Documentation

### Quick Start
- **[README.md](../README.md)** - Project overview, installation, and basic usage
  - Quick start guide
  - Feature highlights
  - Examples (CLI and GUI)
  - Lua scripting reference
  - Troubleshooting

### GUI Application Guide
- **[GUIDE.md](GUIDE.md)** - Complete GUI user guide
  - Feature overview
  - Keyboard shortcuts
  - Data export
  - Settings management
  - Tips and tricks

## Developer Documentation

### Development Guide
- **[DEVELOPMENT.md](../DEVELOPMENT.md)** - Developer guide
  - Development setup
  - Build commands
  - Testing guide
  - Code quality standards
  - Project structure
  - Architecture overview

### Implementation Details
- **[IMPLEMENTATION_SUMMARY.md](../IMPLEMENTATION_SUMMARY.md)** - Implementation progress
- **[COMPLETION_REPORT.md](../COMPLETION_REPORT.md)** - Feature completion report
- **[FINAL_COMPLETION_REPORT.md](../FINAL_COMPLETION_REPORT.md)** - Final status

### Task Tracking
- **[TODO_UI.md](../TODO_UI.md)** - UI improvement tasks and status

## Reference Documentation

### API Documentation
- **Serial Port API** - Port management commands
- **Protocol API** - Protocol system reference
- **Lua Scripting API** - Embedded Lua functions

### Technical Specifications
- **Tauri Commands** - Backend command reference
- **Type Definitions** - TypeScript/Rust type definitions
- **Event System** - Tauri event reference

## Changelog

- **[CHANGELOG.md](../CHANGELOG.md)** - Version history and changes

## Support

### Troubleshooting
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - Detailed troubleshooting guide
  - Common issues and solutions
  - Platform-specific problems
  - Debug mode

## Quick Reference

### CLI Commands
```bash
# List ports
serial-cli list-ports

# Interactive mode
serial-cli interactive

# Run script
serial-cli run script.lua --port=/dev/ttyUSB0

# Send data
serial-cli send --port=/dev/ttyUSB0 "AT\r\n"
```

### GUI Commands
```bash
# Start GUI
just gui-dev

# Build GUI
just gui-build

# Type check
just gui-type-check
```

### Build Commands
```bash
# Development
just dev

# Release
just build

# Test
just test

# All checks
just check
```

## Documentation Status

| Document | Status | Last Updated |
|----------|--------|--------------|
| README.md | ✅ Complete | 2025-04-09 |
| DEVELOPMENT.md | ✅ Complete | 2025-04-09 |
| CHANGELOG.md | ✅ Complete | 2025-04-09 |
| GUIDE.md | ✅ Complete | 2025-04-09 |
| TROUBLESHOOTING.md | ✅ Available | - |
| TODO_UI.md | ✅ Complete | 2025-04-09 |
| Implementation Reports | ✅ Complete | 2025-04-09 |

---

**Documentation Version**: 0.2.0
**Last Updated**: 2025-04-09
