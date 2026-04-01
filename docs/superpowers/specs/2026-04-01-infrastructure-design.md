# Serial CLI 基础设施完善设计文档

**日期:** 2026-04-01
**作者:** Claude
**状态:** 已批准

---

## 1. 概述

为 Serial CLI 项目添加完善的基础设施，包括 CI/CD、交叉编译、构建工具和文档组织。

**设计目标：**
- 简单实用，易于维护
- 支持多平台交叉编译（Windows、多架构 Linux、macOS）
- 自动化测试和质量检查
- 清晰的文档组织

---

## 2. 项目结构

```
serial-cli/
├── .github/
│   └── workflows/
│       └── ci.yml              # CI 配置（测试 + 检查）
├── .cargo/
│   └── config.toml             # Cargo 优化配置
├── justfile                    # Just 命令定义
├── README.md                   # 简化的快速开始
├── USAGE.md                    # 详细使用说明（新建）
├── Cargo.toml                  # 现有（添加交叉编译配置）
└── ...（其他现有文件保持不变）
```

---

## 3. CI/CD 设计

### 3.1 GitHub Actions Workflow

**文件:** `.github/workflows/ci.yml`

**触发条件:**
- Push to `main` or `master` branch
- Pull requests to any branch

**构建矩阵:**
```yaml
os: [ubuntu-latest, macos-latest, windows-latest]
rust: [stable]
```

**检查步骤:**
1. **代码格式检查** - `cargo fmt -- --check`
2. **Clippy Lint** - `cargo clippy -- -D warnings`
3. **单元测试** - `cargo test --all-features`
4. **文档测试** - `cargo test --doc` (仅 Linux)

**预期运行时间:** 2-5 分钟

### 3.2 CI 覆盖范围

- ✅ 代码风格一致性
- ✅ 常见错误和警告（Clippy）
- ✅ 跨平台测试（Linux, macOS, Windows）
- ✅ 文档示例测试

---

## 4. 交叉编译策略

### 4.1 工具选择

**使用 `cross` 工具：**
- 自动处理交叉编译环境
- 通过 Docker 提供 Linux 目标的编译环境
- 简化多架构编译流程

**安装:**
```bash
cargo install cross
```

### 4.2 目标平台

**Linux:**
- `x86_64-unknown-linux-gnu` (默认)
- `aarch64-unknown-linux-gnu` (ARM64)
- `armv7-unknown-linux-gnueabihf` (ARM v7)

**macOS:**
- `x86_64-apple-darwin` (Intel)
- `aarch64-apple-darwin` (Apple Silicon)

**Windows:**
- `x86_64-pc-windows-msvc` (MSVC)

### 4.3 Justfile 命令

```bash
just build-all          # 构建所有平台
just build-linux        # Linux x86_64 + aarch64
just build-macos        # macOS x86_64 + arm64
just build-windows      # Windows x86_64
just release            # 构建所有平台 release
just clean              # 清理所有构建产物
```

### 4.4 交叉编译实现

**Linux 目标（使用 cross）:**
```bash
cross build --target aarch64-unknown-linux-gnu --release
cross build --target armv7-unknown-linux-gnueabihf --release
```

**macOS 目标:**
- 需要在 macOS 上原生编译
- 或使用 `osxcross` 工具链（Linux 上）

**Windows 目标:**
- Linux 上使用 `cross` + mingw
- 或在 Windows 上原生编译

---

## 5. Cargo 配置优化

### 5.1 .cargo/config.toml

**优化内容:**
```toml
[build]
# 并行编译
jobs = 4

[target.x86_64-unknown-linux-gnu]
# Linux 特定配置

[target.aarch64-unknown-linux-gnu]
# ARM64 Linux 配置

[target.x86_64-apple-darwin]
# macOS Intel 配置

[target.aarch64-apple-darwin]
# macOS Apple Silicon 配置

[target.x86_64-pc-windows-msvc]
# Windows MSVC 配置
```

### 5.2 Cargo.toml 增强

**添加交叉编译相关元数据:**
```toml
[package.metadata.cross]
# 为 cross 工具提供提示
```

---

## 6. 文档重组

### 6.1 README.md（简化版）

**保留内容:**
- 项目简介和目标
- 核心特性列表
- 快速开始指南
  - 安装说明
  - 基本用法示例
- 文档链接
- 项目状态徽章

**删除内容:**
- 详细用法说明（移到 USAGE.md）
- 完整 API 文档（移到 USAGE.md）
- 重复的项目结构说明

### 6.2 USAGE.md（新建）

**详细内容包括:**

1. **命令参考**
   - `list-ports` - 列出串口
   - `interactive` - 交互模式
   - `send` - 发送数据
   - `recv` - 接收数据
   - `status` - 查询状态
   - `run` - 运行 Lua 脚本
   - `batch` - 批处理

2. **交互模式完整指南**
   - 所有命令详解
   - 工作流程示例
   - 高级用法

3. **Lua 脚本 API**
   - 串口 API (serial:send, serial:recv)
   - 日志 API (log_info, log_debug, etc.)
   - 工具函数 (sleep_ms, string_to_hex, etc.)
   - JSON 处理

4. **协议配置**
   - Modbus RTU
   - Modbus ASCII
   - AT Command
   - Line-based
   - 自定义 Lua 协议

5. **批处理模式**
   - 顺序执行
   - 并行执行
   - 示例脚本

6. **故障排除**
   - 常见问题
   - 调试技巧
   - 权限问题

7. **示例脚本**
   - 基本 I/O
   - Modbus 通信
   - AT 命令
   - 自定义协议

### 6.3 文档结构

```
docs/
├── README.md           # 概览和快速开始
├── USAGE.md            # 详细使用说明
├── IMPLEMENTATION_STATUS.md  # 实现状态
├── TROUBLESHOOTING.md  # 故障排除
└── examples/           # 示例脚本
```

---

## 7. Justfile 设计

### 7.1 核心命令

```bash
# 开发相关
just dev                 # 开发构建
just build               # Release 构建
just test                # 运行测试
just test-verbose        # 详细测试输出
just test-watch          # 监听模式测试

# 代码质量
just lint                # 运行 clippy
just fmt                 # 格式化代码
just fmt-check           # 检查格式
just check               # 运行所有检查（fmt + lint + test）

# 清理
just clean               # 清理构建产物
just clean-all           # 清理所有（包括 target/）

# 文档
just docs                # 生成并打开文档
just docs-check          # 检查文档链接

# 交叉编译
just build-linux         # Linux x86_64 + aarch64
just build-macos         # macOS x86_64 + arm64
just build-windows       # Windows x86_64
just build-all           # 构建所有平台
just release             # 构建所有平台 release

# 安装
just install             # 本地安装
just install-release     # 安装 release 版本
```

### 7.2 Justfile 特性

- 使用 `just` 的递归和默认特性
- 支持命令参数传递
- 依赖管理（如 `release` 依赖 `clean`）
- 跨平台兼容性（Windows 使用不同的命令）

---

## 8. 实施步骤

### 阶段 1: 基础配置
1. 创建 `.cargo/config.toml`
2. 创建 `justfile` 并测试命令
3. 更新 `Cargo.toml` 添加元数据

### 阶段 2: CI/CD
1. 创建 `.github/workflows/ci.yml`
2. 测试 CI 在所有平台通过
3. 添加状态徽章到 README

### 阶段 3: 文档重组
1. 简化 `README.md`
2. 创建 `USAGE.md` 并迁移详细内容
3. 更新文档链接

### 阶段 4: 交叉编译
1. 安装和配置 `cross`
2. 实现 Justfile 交叉编译命令
3. 测试所有目标平台编译
4. 添加交叉编译文档

### 阶段 5: 验证和文档
1. 运行完整 CI 流程
2. 测试所有 just 命令
3. 验证交叉编译产物
4. 更新相关文档

---

## 9. 成功标准

- ✅ CI 在所有平台通过测试
- ✅ Justfile 所有命令正常工作
- ✅ 能够交叉编译到所有目标平台
- ✅ 文档清晰易读，结构合理
- ✅ 新用户可以快速上手
- ✅ 维护者可以轻松构建和发布

---

## 10. 未来扩展（可选）

**不在本次实施范围：**
- 自动发布到 GitHub Releases
- Homebrew formula 自动生成
- AUR package 支持
- Snap/AppImage 打包
- CI 性能优化（缓存、并行）
- 集成测试覆盖率报告

---

## 附录 A: 目标平台详细列表

| 平台 | 目标三元组 | 优先级 | 备注 |
|------|-----------|--------|------|
| Linux (x86_64) | x86_64-unknown-linux-gnu | P0 | 主要平台 |
| Linux (ARM64) | aarch64-unknown-linux-gnu | P1 | 服务器/嵌入式 |
| Linux (ARM v7) | armv7-unknown-linux-gnueabihf | P2 | 树莓派等 |
| macOS (Intel) | x86_64-apple-darwin | P0 | 主要平台 |
| macOS (Apple Silicon) | aarch64-apple-darwin | P0 | 主要平台 |
| Windows (x86_64) | x86_64-pc-windows-msvc | P0 | 主要平台 |

---

## 附录 B: 工具版本要求

- Rust: 1.70+ (stable)
- Just: 1.0+
- Cross: 0.2+
- Docker: 20+ (用于 cross)
- Git: 2.0+

---

**设计完成，等待实施。**
