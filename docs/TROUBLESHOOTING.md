# Serial CLI - Troubleshooting Guide

## Common Issues and Solutions

### 1. Port Permission Denied

**Error:**
```
Error: Permission denied for port '/dev/ttyUSB0'
```

**Solutions:**
- Linux: Add user to `dialout` group: `sudo usermod -a -G dialout $USER`
- macOS: No special permissions usually needed
- Windows: Ensure no other application is using the port

### 2. Port Not Found

**Error:**
```
Error: Port '/dev/ttyUSB0' not found
```

**Solutions:**
- Check connection: `serial-cli list-ports`
- Try different port name
- Check USB cable and device power

### 3. Timeout Errors

**Error:**
```
Error: Operation timeout
```

**Solutions:**
- Increase timeout in configuration: `timeout_ms = 5000`
- Check if device is responding
- Verify baud rate and other settings
- Try sending a simple command first (like "AT")

### 4. Protocol Errors

**Error:**
```
Error: Checksum mismatch
```

**Solutions:**
- Verify protocol settings match device configuration
- Check for noise in communication
- Try lower baud rate for better reliability
- Enable debug mode: `serial-cli --verbose`

### 5. Lua Script Errors

**Error:**
```
Error: Runtime error in script.lua
```

**Solutions:**
- Check Lua syntax: `lua -e "dofile('script.lua')"`
- Enable verbose mode: `serial-cli --verbose run script.lua`
- Validate API calls
- Check script with `lua -c script.lua`

## Debug Mode

Enable verbose logging:
```bash
serial-cli --verbose list-ports
serial-cli --verbose send "AT+CMD"
```

## Getting Help

### Command Help
```bash
serial-cli --help
serial-cli send --help
serial-cli list-ports --help
```

### Interactive Shell Help
```bash
serial-cli interactive
serial> help
```

## Performance Issues

### Slow Communication
- Try increasing buffer sizes in configuration
- Disable unnecessary logging
- Use appropriate baud rate

### High CPU Usage
- Check for polling loops
- Use async operations instead of busy waiting
- Reduce logging verbosity

## Additional Resources

- GitHub Issues: https://github.com/yourusername/serial-cli/issues
- Documentation: See README.md and docs/
- Examples: Check examples/ directory
