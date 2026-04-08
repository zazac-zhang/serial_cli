# Serial CLI - 使用指南

完整的使用文档，涵盖所有命令和 Lua API。

## 快速开始

### 安装

```bash
# 从源码安装
cargo install --path .

# 或下载预构建二进制
# https://github.com/zazac-zhang/serial_cli/releases
```

### 基本用法

```bash
# 列出可用端口
serial-cli list-ports

# 交互模式
serial-cli interactive

# 发送数据
serial-cli send --port=/dev/ttyUSB0 "AT+CMD"

# 运行 Lua 脚本
serial-cli run script.lua --port=/dev/ttyUSB0
```

## 命令参考

### 全局选项

- `--json`: JSON 格式输出
- `-v, --verbose`: 详细日志

### 可用命令

#### `list-ports`
列出所有可用串口。

```bash
serial-cli list-ports
```

#### `interactive`
启动交互式 REPL。

```bash
serial-cli interactive
```

#### `send`
发送数据到串口。

```bash
serial-cli send --port=<PORT> <DATA>
```

#### `run`
执行 Lua 脚本。

```bash
serial-cli run <SCRIPT_FILE>
```

#### `batch`
批量执行多个脚本。

```bash
# 顺序执行
serial-cli batch script1.lua script2.lua

# 并行执行
serial-cli batch --parallel script1.lua script2.lua
```

## 交互模式

```bash
$ serial-cli interactive
Serial CLI Interactive Shell
Type 'help' for available commands, 'quit' to exit

serial> list              # 列出端口
serial> open /dev/ttyUSB0 # 打开端口
serial> send "AT\r\n"     # 发送数据
serial> recv              # 接收数据
serial> status            # 查看状态
serial> close             # 关闭端口
serial> quit              # 退出
```

## Lua 脚本 API

### 串口操作

```lua
-- 打开串口
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    timeout = 1000,
    data_bits = 8,
    parity = "none",
    stop_bits = 1
})

-- 写入数据
port:write("Hello\r\n")

-- 读取数据
local data = port:read(256)
local line = port:read_until("\n")

-- 关闭端口
port:close()
```

### 日志函数

```lua
log_info("信息")
log_warn("警告")
log_error("错误")
log_debug("调试")
```

### 工具函数

```lua
-- 延时
sleep_ms(1000)

-- 十六进制转换
local hex = string_to_hex("ABC")      -- "414243"
local str = hex_to_string("414243")   -- "ABC"

-- 时间戳
local ts = timestamp_ms()
```

### 协议工具

```lua
-- Modbus RTU
local modbus = serial.protocols.modbus.new(port, {
    device_id = 1,
    timeout = 1000
})

local registers = modbus:read_holding_registers(0x0000, 10)
```

### 自定义协议

```lua
-- 加载自定义协议
local ok, err = protocol_load("/path/to/custom.lua")
if ok then
    local encoded = protocol_encode("my_protocol", "data")
    local decoded = protocol_decode("my_protocol", encoded)
end
```

## Lua 脚本示例

### AT 命令示例

```lua
log_info("AT 命令测试")

local port = serial.open("/dev/ttyUSB0", { baudrate = 115200 })

port:write("AT\r\n")
sleep_ms(100)
local resp = port:read_until(string.byte("\n"))
log_info("响应：" .. resp)

port:close()
```

### Modbus RTU 示例

```lua
log_info("Modbus RTU 测试")

local port = serial.open("/dev/ttyUSB0", { baudrate = 9600 })

-- 读取保持寄存器（功能码 0x03）
local request = string.char(
    0x01, 0x03, 0x00, 0x00, 0x00, 0x0A
)

port:write(request)
sleep_ms(100)
local response = port:read(100)

log_info("响应：" .. string_to_hex(response))
port:close()
```

## 常见问题

### 权限被拒绝（Linux）

```bash
# 添加用户到 dialout 组
sudo usermod -a -G dialout $USER
# 重新登录或执行
newgrp dialout
```

### 端口未找到

- 检查设备连接
- 使用 `list-ports` 确认可用端口
- 检查驱动程序是否安装

### 超时错误

- 增加超时设置：`timeout = 5000`
- 确认波特率与设备匹配
- 检查线缆连接

## 更多信息

- [README.md](README.md) - 项目概述
- [DEVELOPMENT.md](DEVELOPMENT.md) - 开发指南
- [GitHub Issues](https://github.com/zazac-zhang/serial_cli/issues) - 问题反馈
