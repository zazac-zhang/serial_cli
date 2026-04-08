# Serial CLI - 开发指南

开发者贡献文档。

## 目录

- [开发环境](#开发环境)
- [构建](#构建)
- [测试](#测试)
- [代码质量](#代码质量)
- [交叉编译](#交叉编译)
- [贡献](#贡献)
- [项目结构](#项目结构)

## 开发环境

### 前置条件

```bash
# Rust 1.70+
rustup update stable
rustup component add rustfmt clippy

# Just 任务运行器（推荐）
cargo install just
```

### IDE 设置

**VS Code 推荐扩展:**
- rust-analyzer
- CodeLLDB
- Even Better TOML
- Error Lens

**.vscode/settings.json:**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true
}
```

## 构建

```bash
# 开发构建
just dev        # cargo build

# Release 构建
just build      # cargo build --release

# 运行
just run <args> # cargo run -- <args>
```

## 测试

```bash
# 所有测试
just test

# 详细输出
just test-verbose

# 特定测试
just test <test_name>
```

**测试状态:** 58/58 passing ✅

## 代码质量

```bash
# 格式化
just fmt         # cargo fmt
just fmt-check   # cargo fmt -- --check

# Lint
just lint        # cargo clippy -- -D warnings

# 全部检查
just check       # fmt-check + lint + test
```

提交前必须通过所有检查。

## 交叉编译

### 前置条件

```bash
# 安装 cross
cargo install cross

# Docker (cross 需要)
```

### 构建命令

```bash
# 所有平台
just build-all

# 特定平台
just build-linux    # x86_64 + aarch64
just build-macos    # x86_64 + arm64 (仅 macOS)
just build-windows  # x86_64 (需要 cross)
```

### 平台说明

**Linux:**
```bash
sudo apt-get install build-essential libudev-dev
sudo usermod -a -G dialout $USER
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- 安装 Visual Studio Build Tools（C++ 工具）

## 贡献

### 贡献类型

- 🐛 Bug 修复
- ✨ 新功能
- 🧪 测试
- 📚 文档
- 🔧 性能优化

### 流程

```bash
# 1. 创建分支
git checkout -b feature/your-feature-name

# 2. 运行检查
just check

# 3. 提交
git commit -m "Add: Your feature description"

# 4. 创建 PR
```

### Commit 规范

```
<类型>: <简短描述>
```

**类型:** `Add:`, `Fix:`, `Update:`, `Refactor:`, `Docs:`, `Test:`, `Chore:`, `Perf:`

### 代码风格

- 使用 `cargo fmt` 格式化
- 修复所有 `clippy` 警告
- 新功能需要单元测试
- 复杂逻辑需要注释

## 项目结构

```
serial-cli/
├── src/
│   ├── main.rs              # CLI 入口
│   ├── lib.rs               # 库入口
│   ├── error.rs             # 错误类型
│   ├── config.rs            # 配置管理
│   ├── serial_core/         # 串口核心
│   │   ├── port.rs          # 端口管理
│   │   ├── io_loop.rs       # 异步 I/O
│   │   └── sniffer.rs       # 数据包捕获
│   ├── protocol/            # 协议引擎
│   │   ├── registry.rs      # 协议注册表
│   │   ├── built_in/        # 内置协议 (Modbus, AT, Line)
│   │   └── lua_ext.rs       # Lua 协议扩展
│   ├── lua/                 # LuaJIT 运行时
│   │   ├── engine.rs        # Lua 引擎
│   │   ├── bindings.rs      # API 绑定
│   │   └── executor.rs      # 脚本执行器
│   ├── task/                # 任务调度
│   └── cli/                 # CLI 接口
│       ├── interactive.rs   # REPL
│       ├── commands.rs      # 单命令
│       └── batch.rs         # 批量处理
├── examples/                # Lua 脚本示例
├── tests/                   # 集成测试
├── docs/                    # 文档
├── justfile                 # Just 命令
├── Cargo.toml               # 包配置
├── README.md                # 快速开始
├── USAGE.md                 # 使用指南
└── DEVELOPMENT.md           # 本文档
```

### 核心模块

| 模块 | 描述 |
|------|------|
| `serial_core` | 串口 I/O，端口管理 |
| `protocol` | Modbus, AT 命令，自定义协议 |
| `lua` | LuaJIT 集成，脚本执行 |
| `cli` | 命令行界面，交互模式 |

## 调试

```bash
# 调试日志
RUST_LOG=debug cargo run -- list-ports
RUST_LOG=trace cargo run -- list-ports

# 性能分析
cargo install flamegraph
cargo flamegraph --bin serial-cli -- list-ports

# Benchmark
cargo bench
```

## 更多资源

- [Rust 指南](https://rust-lang.github.io/api-guidelines/)
- [API 文档](https://docs.rs/serial-cli/)
- [USAGE.md](USAGE.md) - 使用文档
- [GitHub Issues](https://github.com/zazac-zhang/serial_cli/issues)
