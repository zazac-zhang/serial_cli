# Serial CLI - Just 命令配置
# https://github.com/casey/just

# 默认列表（运行 `just` 时显示）
default:
    @just --list

# =============================================================================
# 开发命令
# =============================================================================

# 开发构建（未优化）
dev:
    cargo build

# Release 构建（优化）
build:
    cargo build --release

# 运行项目（开发模式）
run *args:
    cargo run -- {{args}}

# =============================================================================
# 测试命令
# =============================================================================

# 运行所有测试
test:
    cargo test

# 运行测试（详细输出）
test-verbose:
    cargo test -- --nocapture

# 运行测试并显示输出
test-watch:
    cargo watch -x test

# 运行特定测试
test-test name *args:
    cargo test {{name}} -- {{args}}

# =============================================================================
# 代码质量
# =============================================================================

# 运行 Clippy 检查
lint:
    cargo clippy -- -D warnings

# 格式化代码
fmt:
    cargo fmt

# 检查代码格式
fmt-check:
    cargo fmt -- --check

# 运行所有检查（格式 + lint + 测试）
check: fmt-check lint test

# =============================================================================
# 清理命令
# =============================================================================

# 清理构建产物
clean:
    cargo clean

# 清理所有（包括 target/）
clean-all:
    rm -rf target/

# =============================================================================
# 文档命令
# =============================================================================

# 生成并打开文档
docs:
    cargo doc --open

# 生成文档（不打开）
docs-build:
    cargo doc

# 检查文档链接
docs-check:
    cargo doc --document-private-items

# =============================================================================
# 交叉编译
# =============================================================================

# 构建所有平台
build-all: build-linux build-macos build-windows
    @echo "✓ All platforms built successfully"

# 构建 Linux 平台（x86_64 + aarch64）
build-linux:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building Linux x86_64..."
    cargo build --release --target x86_64-unknown-linux-gnu
    echo "Building Linux aarch64..."
    if command -v cross &> /dev/null; then
        cross build --release --target aarch64-unknown-linux-gnu
    else
        echo "⚠ Warning: 'cross' not installed. Install with: cargo install cross"
        echo "Skipping aarch64 build"
    fi

# 构建 macOS 平台（仅限 macOS）
build-macos:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Building macOS x86_64..."
        cargo build --release --target x86_64-apple-darwin
        echo "Building macOS arm64..."
        cargo build --release --target aarch64-apple-darwin
    else
        echo "⚠ macOS builds can only be performed on macOS"
        echo "Skipping macOS build"
    fi

# 构建 Windows 平台（使用 cross）
build-windows:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building Windows x86_64..."
    if command -v cross &> /dev/null; then
        cross build --release --target x86_64-pc-windows-msvc
    else
        echo "⚠ Warning: 'cross' not installed. Install with: cargo install cross"
        echo "Skipping Windows build"
    fi

# 构建所有平台的 Release 版本
release: clean-all build-all
    @echo "✓ Release builds complete"

# =============================================================================
# GUI 开发命令
# =============================================================================

# 安装 GUI 依赖
gui-deps:
    cd src-ui && npm install

# 启动前端开发服务器
gui-dev-frontend:
    cd src-ui && npm run dev

# 启动 GUI 应用（开发模式）
gui-dev:
    cd src-ui && npm run dev &
    sleep 2
    cargo tauri dev

# 构建 GUI 应用
gui-build:
    cd src-ui && npm run build
    cargo tauri build

# 构建 GUI 前端
gui-build-frontend:
    cd src-ui && npm run build

# 检查 GUI 前端类型
gui-type-check:
    cd src-ui && npm run type-check

# 运行 GUI 测试
gui-test:
    cd src-ui && npm test

# 清理 GUI 构建产物
gui-clean:
    rm -rf src-ui/dist
    rm -rf src-ui/node_modules
    rm -rf src-tauri/target

# 检查 GUI Rust 代码
gui-check:
    cargo check --workspace

# 格式化 GUI 代码
gui-fmt:
    cargo fmt
    cd src-ui && npx prettier --write "src/**/*.{ts,tsx,css}"

# =============================================================================
# 安装命令
# =============================================================================

# 本地安装（开发版本）
install:
    cargo install --path .

# 本地安装（Release 版本）
install-release: build
    cargo install --path .
