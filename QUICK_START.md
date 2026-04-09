# Serial CLI 快速开始指南

## 安装和构建

```bash
# 克隆项目
git clone <repository-url>
cd serial_cli

# 开发版本构建
just dev

# 发布版本构建
just build

# 运行测试
just test
```

## 基础使用

### 1. 列出可用端口

```bash
serial-cli list-ports
```

### 2. 发送数据

```bash
# 简单发送
serial-cli send --port /dev/ttyUSB0 "Hello, World!"

# 发送十六进制数据
serial-cli send --port /dev/ttyUSB0 "0102030405"
```

### 3. 交互式模式

```bash
serial-cli interactive
```

在交互式 shell 中：
```
serial> list              # 列出端口
serial> open /dev/ttyUSB0 # 打开端口
serial> send Hello        # 发送数据
serial> recv 64           # 接收数据
serial> status            # 查看状态
serial> quit              # 退出
```

## 高级功能

### 配置管理

```bash
# 查看当前配置
serial-cli config show

# 修改配置
serial-cli config set serial.baudrate 9600
serial-cli config set serial.timeout_ms 2000
serial-cli config set logging.level debug

# 保存配置
serial-cli config save

# 查看配置文件
cat .serial-cli.toml
```

### Lua 脚本

```bash
# 执行简单脚本
serial-cli run script.lua

# 带参数执行
serial-cli run script.lua arg1 arg2 arg3
```

Lua 脚本示例：
```lua
-- script.lua
print("Arguments received:")
for i = 1, #arg do
    print(string.format("arg[%d] = %s", i, arg[i]))
end

-- 打开端口
local port = serial_open("/dev/ttyUSB0")
serial_send(port, "AT\r\n")
local response = serial_recv(port, 64)
print("Response:", response)
serial_close(port)
```

### 嗅探功能

```bash
# 启动嗅探（捕获 1000 个数据包）
serial-cli sniff start /dev/ttyUSB0 --max-packets 1000

# 启动嗅探并保存到文件
serial-cli sniff start /dev/ttyUSB0 --output capture.log

# 查看嗅探统计
serial-cli sniff stats

# 保存捕获的数据
serial-cli sniff save output.log
```

### 批处理

```bash
# 运行批处理脚本
serial-cli batch run scripts.txt

# 并发运行（最多 3 个并发任务）
serial-cli batch run scripts.txt --concurrent 3

# 列出批处理脚本
serial-cli batch list
```

批处理脚本格式（scripts.txt）：
```
# 注释行以 # 开头
script1.lua
script2.lua
script3.lua
```

### 协议管理

```bash
# 列出所有协议
serial-cli protocol list

# 查看协议信息
serial-cli protocol info modbus_rtu

# 验证协议脚本
serial-cli protocol validate custom_protocol.lua
```

## 配置文件

### 默认配置文件位置
- 项目级：`.serial-cli.toml`
- 全局级：`~/.config/serial-cli/config.toml` (Linux/macOS)
- 全局级：`%APPDATA%\serial-cli\config.toml` (Windows)

### 配置文件示例

```toml
[serial]
baudrate = 115200
databits = 8
stopbits = 1
parity = "none"
timeout_ms = 1000

[logging]
level = "info"
format = "text"
file = ""

[lua]
memory_limit_mb = 128
timeout_seconds = 300
enable_sandbox = true

[task]
max_concurrent = 10
default_timeout_seconds = 60

[output]
json_pretty = true
show_timestamp = true
```

## 错误处理和故障排除

### 常见错误和解决方案

#### 权限拒绝
```
❌ Error: Permission denied: /dev/ttyUSB0
```
**解决方案：**
```bash
# Linux/macOS
sudo serial-cli send --port /dev/ttyUSB0 "test"

# 或者将用户添加到 dialout 组
sudo usermod -a -G dialout $USER
```

#### 端口未找到
```
❌ Error: Port not found: /dev/ttyUSB99
```
**解决方案：**
```bash
# 先列出可用端口
serial-cli list-ports

# 使用正确的端口名
serial-cli send --port /dev/ttyUSB0 "test"
```

#### 端口忙碌
```
❌ Error: Port busy: /dev/ttyUSB0
```
**解决方案：**
```bash
# 关闭其他使用该端口的程序
# Linux: 查找占用端口的进程
lsof | grep ttyUSB0

# Windows: 在设备管理器中检查端口使用情况
```

#### 超时错误
```
❌ Error: Operation timeout
```
**解决方案：**
```bash
# 增加超时时间
serial-cli config set serial.timeout_ms 5000
serial-cli config save
```

## 高级用法

### 1. 自定义协议

创建自定义 Lua 协议：

```lua
-- custom_protocol.lua
local protocol = {}

function protocol.parse(data)
    -- 解析接收到的数据
    return parsed_data
end

function protocol.encode(data)
    -- 编码要发送的数据
    return encoded_data
end

function protocol.reset()
    -- 重置协议状态
end

return protocol
```

### 2. 性能监控

```bash
# 启用性能监控
serial-cli --performance run script.lua

# 查看性能报告
# （功能在代码中实现，可通过 API 调用）
```

### 3. 自动重连

```bash
# 使用自动重连功能
# （在代码中实现，可通过配置启用）
```

## 开发者指南

### 添加新的内置协议

1. 在 `src/protocol/built_in/` 中创建新的协议文件
2. 实现 `Protocol` trait
3. 在协议注册表中注册

### 扩展 Lua API

1. 在 `src/lua/bindings.rs` 中添加新的 API 函数
2. 注册到 Lua 全局命名空间
3. 添加文档和示例

### 贡献代码

```bash
# 运行代码质量检查
just check

# 运行所有测试
just test-verbose

# 格式化代码
just fmt

# 运行 linter
just lint
```

## 常用命令速查表

| 命令 | 描述 | 示例 |
|------|------|------|
| `list-ports` | 列出可用端口 | `serial-cli list-ports` |
| `send` | 发送数据 | `serial-cli send --port /dev/ttyUSB0 "data"` |
| `interactive` | 交互式模式 | `serial-cli interactive` |
| `run` | 运行 Lua 脚本 | `serial-cli run script.lua` |
| `config show` | 显示配置 | `serial-cli config show` |
| `config set` | 设置配置 | `serial-cli config set serial.baudrate 9600` |
| `sniff start` | 开始嗅探 | `serial-cli sniff start /dev/ttyUSB0` |
| `batch run` | 运行批处理 | `serial-cli batch run scripts.txt` |
| `protocol list` | 列出协议 | `serial-cli protocol list` |

## 获取帮助

```bash
# 查看帮助信息
serial-cli --help

# 查看子命令帮助
serial-cli send --help
serial-cli config --help
```

## 更多资源

- 项目文档：`README.md`
- API 文档：生成 rustdoc
- 配置参考：查看 `src/config.rs`
- 协议开发：查看 `src/protocol/`
- Lua API：查看 `src/lua/bindings.rs`

---

**祝您使用愉快！** 🚀

如有问题或建议，欢迎提交 Issue 或 Pull Request。
