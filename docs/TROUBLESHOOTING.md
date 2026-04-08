# 故障排除指南

## 常见问题

### 1. 权限被拒绝

**错误:** `Permission denied for port '/dev/ttyUSB0'`

**解决方案:**

**Linux:**
```bash
# 添加用户到 dialout 组
sudo usermod -a -G dialout $USER
# 重新登录或执行
newgrp dialout
```

**Windows:**
- 以管理员身份运行命令行
- 关闭其他使用该端口的应用

### 2. 端口未找到

**错误:** `Port '/dev/ttyUSB0' not found`

**解决方案:**
- 使用 `serial-cli list-ports` 确认可用端口
- 检查 USB 连接和线缆
- Windows: 在设备管理器中查看 COM 端口

### 3. 超时错误

**错误:** `Operation timeout`

**解决方案:**
- 增加超时：`timeout = 5000`
- 确认波特率与设备匹配
- 检查设备是否响应

### 4. 端口被占用

**错误:** `Port 'COM1' is already in use`

**解决方案:**
- 关闭 PuTTY、Tera Term、Arduino IDE 等
- 在设备管理器中禁用后重新启用端口

### 5. Lua 脚本错误

**错误:** `Runtime error in script.lua`

**解决方案:**
- 使用 `--verbose` 查看详细错误
- 验证 Lua 语法
- 检查 API 调用是否正确

## 调试模式

```bash
# 启用详细日志
serial-cli --verbose list-ports
serial-cli --verbose run script.lua

# 设置日志级别
RUST_LOG=debug serial-cli list-ports
RUST_LOG=trace serial-cli list-ports
```

## 平台特定问题

### Linux

**依赖安装:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential libudev-dev

# Fedora/RHEL
sudo dnf install gcc make libudev-devel
```

### macOS

**Xcode 工具:**
```bash
xcode-select --install
```

### Windows

**驱动安装:**
- FTDI、CP210x、CH340 等 USB 转串口驱动
- Arduino IDE 包含常用驱动

**Visual Studio Build Tools (开发需要):**
- 安装时选择 "C++ build tools"
- 包含 Windows SDK

## 获取帮助

- GitHub Issues: https://github.com/zazac-zhang/serial_cli/issues
- 文档：README.md, USAGE.md, DEVELOPMENT.md
