# 虚拟串口快速开始指南

## 🚀 5分钟快速开始

### 第一步：验证安装
```bash
# 检查 serial-cli 是否安装
serial-cli --version

# 查看虚拟串口命令
serial-cli virtual --help
```

### 第二步：创建第一个虚拟串口对
```bash
# 创建虚拟串口对
serial-cli virtual create

# 输出示例:
# ✓ Virtual pair created successfully
#   ID: 60afcfe2-4844-4e81-9a9c-8bc0e1d95779
#   Port A: /dev/ttys014
#   Port B: /dev/ttys015
#   Backend: Pty
#
# Usage examples:
#   Terminal 1: serial-cli interactive --port /dev/ttys014
#   Terminal 2: serial-cli interactive --port /dev/ttys015
```

### 第三步：测试虚拟串口
打开两个新终端：

**终端 1:**
```bash
serial-cli interactive --port /dev/ttys014
# 在交互式 shell 中输入:
# Hello from Port A!
```

**终端 2:**
```bash
serial-cli interactive --port /dev/ttys015
# 你应该能看到:
# Hello from Port A!
# 输入回复:
# Hi from Port B!
```

### 第四步：停止虚拟串口
在第一个终端按 `Ctrl+C` 停止虚拟串口对。

## 📋 常用命令

### 创建虚拟串口
```bash
# 基础创建
serial-cli virtual create

# 带监控创建
serial-cli virtual create --monitor

# 保存监控到文件
serial-cli virtual create --monitor --output traffic.log

# 指定后端
serial-cli virtual create --backend pty
```

### 管理虚拟串口
```bash
# 列出所有虚拟串口对
serial-cli virtual list

# 查看统计信息
serial-cli virtual stats <id>

# 停止虚拟串口对
serial-cli virtual stop <id>
```

## 🔧 配置文件

### 创建配置文件
```bash
# 复制示例配置
cp serial-cli.example.toml .serial-cli.toml

# 编辑配置
nano .serial-cli.toml
```

### 常用配置
```toml
[virtual]
# 默认后端
backend = "pty"

# 启用监控
monitor = true

# 监控格式
monitor_format = "hex"

# 自动清理
auto_cleanup = true
```

## 🧪 测试场景

### 1. 协议测试
```bash
# 创建虚拟串口对
serial-cli virtual create --monitor

# 终端 1: 模拟设备
serial-cli interactive --port /dev/ttys014

# 终端 2: 测试协议
serial-cli interactive --port /dev/ttys015
# 在交互式 shell 中设置协议:
# protocol set modbus_rtu
```

### 2. 数据监控
```bash
# 创建带监控的虚拟串口对
serial-cli virtual create --monitor --output test.log

# 在两个终端中通信
# 监控数据会保存到 test.log
```

### 3. 自动化测试
```bash
# 创建虚拟串口并在后台运行
serial-cli virtual create &
PAIR_ID=$!

# 运行测试
cargo test

# 清理
serial-cli virtual stop $PAIR_ID
```

## 💡 Lua 脚本示例

### 创建虚拟串口
```lua
-- examples/virtual_example.lua
log_info("Creating virtual serial port...")

local result = virtual_create("pty", false)
if result then
    log_info("Port A: " .. result.port_a)
    log_info("Port B: " .. result.port_b)

    -- 现在可以使用这些端口进行测试
    -- serial_open(result.port_a, 115200)
    -- serial_send(port_id, "test data")
end
```

## 🛠️ 故障排除

### 权限错误
```bash
# 如果遇到权限错误，尝试:
sudo serial-cli virtual create

# 或者检查 PTY 权限:
ls -l /dev/ptmx
```

### 端口未显示
```bash
# 检查虚拟串口是否仍在运行:
serial-cli virtual list

# 如果没有，重新创建:
serial-cli virtual create
```

### 清理僵尸端口
```bash
# 停止所有虚拟串口对
serial-cli virtual list
# 对每个 id 运行:
serial-cli virtual stop <id>
```

## 📚 更多资源

- **完整文档**: `docs/VIRTUAL_SERIAL.md`
- **功能指南**: `VIRTUAL_SERIAL_FEATURE.md`
- **修复历史**: `VIRTUAL_SERIAL_FIXES.md`
- **完成总结**: `VIRTUAL_SERIAL_COMPLETION_SUMMARY.md`

## 🎯 下一步

1. **探索协议**: 测试内置协议 (Modbus, AT Command, Line)
2. **编写脚本**: 创建 Lua 脚本自动化测试
3. **集成测试**: 在 CI/CD 管道中使用虚拟串口
4. **贡献代码**: 实现 Windows NamedPipe 后端

---

**提示**: 虚拟串口对在 Ctrl+C 或命令退出时会自动清理，无需手动管理资源！
