# Serial CLI - TODO 任务清单

本文档记录了从代码实际审核得出的功能完成情况和待办事项。

## ✅ 已完成功能

### 核心串口管理
- [x] 串口发现和列表 (`serial_core/port.rs`)
- [x] 串口打开/关闭操作
- [x] 串口读写功能
- [x] 串口配置（波特率、数据位、停止位、校验位）
- [x] 多端口并发管理
- [x] 完善的错误处理（权限、忙、超时、未找到）

### 协议系统
- [x] 协议注册表 (`protocol/registry.rs`)
- [x] 协议工厂模式
- [x] Line 协议 - 基于分隔符 (`protocol/built_in/line.rs`)
- [x] Modbus RTU 协议 - CRC16 校验 (`protocol/built_in/modbus.rs`)
- [x] Modbus ASCII 协议 - LRC 校验
- [x] AT Command 协议 (`protocol/built_in/at_command.rs`)
- [x] 协议统计信息

### Lua 脚本支持
- [x] Lua 引擎封装 (`lua/engine.rs`)
- [x] Lua API 绑定 (`lua/bindings.rs`)
  - [x] 日志 API (log_info, log_debug, log_warn, log_error)
  - [x] 工具 API (json_encode, sleep_ms)
- [x] 脚本执行引擎 (`lua/executor.rs`)

### 任务调度系统
- [x] 优先级任务队列 (`task/queue.rs`)
- [x] 异步任务执行器 (`task/executor.rs`)
- [x] 任务类型支持（脚本、串口操作、自定义）
- [x] 并发控制和限流
- [x] 任务完成状态追踪

### CLI 交互模式
- [x] 交互式 Shell (`cli/interactive.rs`)
  - [x] help 命令
  - [x] list 命令（列出串口）
  - [x] open 命令
  - [x] close 命令
  - [x] send 命令
  - [x] recv 命令
  - [x] status 命令
  - [x] protocol 命令（占位符）
- [x] 单命令执行模式 (`cli/commands.rs`)
  - [x] send 操作
  - [x] recv 操作
  - [x] status 操作
  - [x] JSON 输出支持

### 批处理模式
- [x] 批处理执行器 (`cli/batch.rs`)
- [x] 串行脚本执行
- [x] 并行脚本执行
- [x] 错误处理和超时控制

### 异步 I/O 循环
- [x] 事件驱动 I/O 循环 (`serial_core/io_loop.rs`)
- [x] 多端口异步读写
- [x] 事件通道和分发
- [x] 非阻塞读取

### 数据嗅探
- [x] 串口嗅探器 (`serial_core/sniffer.rs`)
- [x] 数据包捕获（TX/RX）
- [x] 时间戳记录
- [x] 十六进制显示
- [x] 文件保存
- [x] 统计信息

### 配置和错误处理
- [x] TOML 配置文件支持 (`config.rs`)
- [x] 全局配置目录
- [x] 项目级配置
- [x] 完善的错误类型定义 (`error.rs`)
- [x] 结构化日志 (tracing)

---

## 🔨 待完成功能

### 协议扩展
- [ ] Lua 自定义协议加载器 (`protocol/lua_ext.rs` - 文件存在但需实现)
- [ ] 协议热重载
- [ ] 协议状态管理
- [ ] 更多协议实现
  - [ ] CAN Bus
  - [ ] MQTT
  - [ ] 自定义协议

### Lua API 完善
- [x] 串口操作 API ✅
  - [x] `serial_open(port, baudrate)`
  - [x] `serial_close(port_id)`
  - [x] `serial_send(port_id, data)`
  - [x] `serial_recv(port_id, timeout_ms)`
  - [x] `serial_list()`
- [x] 协议工具 API ✅
  - [x] `protocol_encode(name, data)`
  - [x] `protocol_decode(name, data)`
  - [x] `protocol_list()`
  - [x] `protocol_info(name)`
- [x] 数据转换工具 ✅
  - [x] `hex_to_bytes(hex_string)`
  - [x] `bytes_to_hex(byte_array)`
  - [x] `bytes_to_string(byte_array)`
  - [x] `string_to_bytes(string)`
- [x] 脚本执行命令 ✅
  - [x] `serial-cli run script.lua`
- [x] 示例脚本 ✅
  - [x] `examples/raw_echo.lua` - 基础回显
  - [x] `examples/modbus_with_tools.lua` - Modbus RTU
  - [x] `examples/custom_protocol.lua` - 自定义协议
- [x] 集成测试 ✅ (4 个测试)
- [ ] 任务控制 API
  - [ ] `task_submit(script)`
  - [ ] `task_wait(task_id)`
  - [ ] `task_cancel(task_id)`
- [ ] 文件 I/O API
- [ ] JSON 完整支持（decode 函数）
- [ ] 标准库函数扩展
  - [ ] 数学函数
  - [ ] 更多字符串操作
  - [ ] 表操作
  - [ ] 时间日期

### 串口工具
- [ ] 数据转换工具
  - [ ] 十六进制转 ASCII
  - [ ] ASCII 转十六进制
  - [ ] Base64 编解码
- [ ] 数据分析工具
  - [ ] 波形分析
  - [ ] 频率统计
  - [ ] 协议分析
- [ ] 自动重连
- [ ] 数据记录和回放

### CLI 增强
- [ ] 协议命令完整实现
  - [ ] `protocol list` - 列出所有协议
  - [ ] `protocol set <name>` - 设置当前协议
  - [ ] `protocol show` - 显示当前协议状态
- [ ] 嗅探命令
  - [ ] `sniff start <port>` - 开始嗅探
  - [ ] `sniff stop` - 停止嗅探
  - [ ] `sniff save <path>` - 保存捕获
  - [ ] `sniff stats` - 显示统计
- [ ] 批处理命令
  - [ ] `batch run <script>` - 运行批处理
  - [ ] `batch list` - 列出批处理任务
- [ ] 配置命令
  - [ ] `config show` - 显示配置
  - [ ] `config set <key> <value>` - 设置配置
  - [ ] `config save` - 保存配置
- [ ] 命令历史和自动补全
- [ ] 宏定义和执行
- [ ] 脚本化命令序列

### 性能和监控
- [ ] 性能分析
  - [ ] 吞吐量测试
  - [ ] 延迟测试
- [ ] 资源监控
  - [ ] CPU 使用率
  - [ ] 内存使用
  - [ ] 串口缓冲区状态
- [ ] 任务监控器 (`task/monitor.rs` - 文件存在但需实现)
  - [ ] 实时任务状态
  - [ ] 任务依赖管理
  - [ ] 死锁检测

### 测试和文档
- [ ] 单元测试覆盖率提升
- [ ] 集成测试
  - [ ] 真实串口测试
  - [ ] 虚拟串口测试
- [ ] 性能基准测试
- [ ] API 文档
- [ ] 用户手册
- [ ] 示例脚本
- [ ] 教程

### 跨平台和打包
- [ ] Windows 支持
- [ ] macOS 支持
- [ ] Linux 支持
- [ ] 安装包
  - [ ] Homebrew
  - [ ] Scoop
  - [ ] AUR
  - [ ] DEB/RPM
- [ ] Docker 镜像

### 其他功能
- [ ] 插件系统
- [ ] 远程管理接口
- [ ] 数据可视化
- [ ] 协议自动识别
- [ ] 多语言支持

---

## 📊 完成度估算

| 模块 | 完成度 | 说明 |
|------|--------|------|
| 串口管理 | 95% | 核心功能完整，需增加工具函数 |
| 协议系统 | 70% | 基础协议完成，需扩展 |
| Lua 支持 | 85% | ✅ 核心 API 完成（串口+协议+转换），示例完整 ✅ |
| 任务调度 | 85% | 核心完成，监控需实现 |
| CLI 交互 | 70% | 基础命令完成，run 命令已实现 ✅ |
| 批处理 | 75% | 核心完成，错误处理需优化 |
| I/O 循环 | 90% | 基本完成 |
| 嗅探器 | 85% | 核心完成，分析工具需添加 |
| 配置管理 | 90% | 基本完成 |
| 错误处理 | 95% | 完善的错误类型 |
| 测试 | 70% | ✅ 单元+集成测试完整，覆盖率提升 ✅ |
| 文档 | 40% | ✅ 基础文档和示例完成 ✅ |

**总体完成度: ~75%** ⬆️ (+10%)

---

## 🎯 下一步优先级

### ✅ 已完成（2026-04-02）
1. ✅ Lua API 完善 - 串口操作和协议 API
2. ✅ 协议工具 API（encode/decode/list/info）
3. ✅ 数据转换工具（hex/bytes/string）
4. ✅ 脚本执行命令实现
5. ✅ 示例脚本和文档
6. ✅ 集成测试

### 高优先级
1. CLI 协议命令实现
2. 任务监控器实现
3. 更多数据转换工具（Base64）
4. 性能测试和优化

### 中优先级
1. 嗅探和分析工具
2. 数据转换工具
3. 命令历史和自动补全
4. 性能测试

### 低优先级
1. 插件系统
2. 远程管理
3. 数据可视化
4. 多语言支持

---

**最后更新**: 2026-04-02
**基于代码审核**: 所有结论来自实际代码分析

---

## 📝 更新日志

### 2026-04-02 - Lua 脚本系统完整实现 ✅

**新增功能**:
- ✅ Lua 串口 API (serial_open, close, send, recv, list)
- ✅ Lua 协议工具 API (protocol_encode, decode, list, info)
- ✅ Lua 数据转换 API (hex_to_bytes, bytes_to_hex, bytes_to_string, string_to_bytes)
- ✅ 脚本执行命令 (`serial-cli run script.lua`)
- ✅ 3 个完整示例脚本
- ✅ 4 个集成测试

**代码质量**:
- 81 个测试全部通过（77 单元 + 4 集成）
- 所有 Clippy 警告已修复
- 代码格式化完成

**提交记录**:
- `b0b2f75` - Merge branch 'feature/lua-serial-api'
- `a4a881e` - fix: resolve all clippy warnings and format code
- `c9aa2d7` - test: add Lua API integration tests
- `0721508` - docs: add Lua scripting examples and documentation
- `73bbcc4` - feat: implement run command for Lua script execution

**总体完成度**: 65% → 75% (+10%)
