# Serial CLI 构建和发布流程完善设计

**文档版本**: 1.0.0
**创建日期**: 2026-04-06
**状态**: 设计阶段
**作者**: Serial CLI Team

---

## 📋 设计概述

### 目标

为Serial CLI项目建立完善的构建和发布流程，支持CLI和GUI双模式分发，优先完善CLI生态，为开发者和技术用户提供便捷的安装和更新体验。

### 核心原则

1. **渐进式实施**: 分3个阶段逐步完善，每个阶段独立验证
2. **模块化设计**: 工具和脚本可独立使用和测试
3. **安全性**: SHA256校验和，commit签名验证
4. **可观测性**: 每个步骤都有清晰的日志和状态报告
5. **向后兼容**: 保留现有功能，逐步增强

### 用户群体

**主要用户**: 开发者和技术用户
- 嵌入式开发者
- 系统管理员
- 自动化工程师

**次要用户**: 终端用户（通过GUI）

### 发布渠道优先级

**CLI（优先）**:
- Cargo（crates.io）
- GitHub Releases
- Homebrew
- Scoop
- AUR

**GUI（辅助）**:
- 独立发布轨道
- 平台原生安装包

---

## 🎯 实施策略：渐进式方案

### 方案选择理由

选择**渐进式实施（3个阶段）**的原因：

1. **风险控制**: 每个阶段都能独立验证，问题容易定位
2. **学习曲线**: 可以在每个阶段学习和调整
3. **灵活调整**: 根据实际情况调整后续阶段
4. **用户体验**: 逐步改善，而不是一次性大变动

### 总时间线

- **阶段1**: 1-2周
- **阶段2**: 2-3周
- **阶段3**: 1-2周
- **总计**: 6-7周

---

## 📦 阶段1：CI/CD基础 + Conventional Commits（1-2周）

### 目标

建立规范的提交和版本管理基础，为后续阶段提供可靠的版本信息。

### 1.1 Conventional Commits规范

#### 提交信息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### 支持的类型

- `feat`: 新功能
- `fix`: Bug修复
- `docs`: 文档变更
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具变更
- `ci`: CI/CD相关

#### 示例

```bash
# 功能开发
feat(cli): add protocol list command
Implement 'protocol list' to display all available protocols
with their version and status information.

Closes #123

# Bug修复
fix(protocol): handle empty response correctly

# 文档更新
docs(readme): update installation instructions
```

### 1.2 CI/CD增强

#### 新增工作流

**A. `commit-lint.yml`** - 提交信息检查
```yaml
name: Commit Lint

on:
  pull_request:
  push:
    branches: [main, master]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: wagoid/commitlint-github-action@v5
```

**B. `release.yml`优化** - 现有workflow增强
- 添加自动版本号推断（基于commits）
- 集成changelog生成
- 添加发布前检查清单
- 保留现有的多平台构建

**C. `version-bump.yml`** - 自动版本管理
```yaml
name: Version Bump

on:
  workflow_dispatch:
    inputs:
      increment:
        description: 'Version increment type'
        required: true
        default: 'auto'
        type: choice
        options:
          - auto
          - major
          - minor
          - patch

jobs:
  bump:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Bump version
        run: scripts/package/prepare-release.sh
```

### 1.3 工具链选择

#### commitlint - 提交信息检查

```bash
npm install -g @commitlint/cli @commitlint/config-conventional
```

**配置文件**: `.github/commitlint.config.js`
```javascript
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [2, 'always', [
      'feat', 'fix', 'docs', 'style', 'refactor',
      'perf', 'test', 'chore', 'ci'
    ]],
    'scope-enum': [2, 'always', [
      'cli', 'gui', 'serial', 'protocol', 'lua',
      'task', 'config', 'build', 'ci'
    ]],
    'type-case': [2, 'always', 'lower-case'],
    'subject-case': [2, 'always', 'sentence-case'],
    'subject-empty': [2, 'never'],
    'subject-full-stop': [2, 'never', '.'],
    'header-max-length': [2, 'always', 100]
  }
};
```

#### git-cliff - Changelog生成器（推荐）

- 基于模板生成
- 支持Conventional Commits
- Rust原生，性能好

**配置文件**: `cliff.toml`
```toml
[changelog]
header = """
# Changelog
All notable changes to this project will be documented in this file.

"""
body = """
{% if version %}\
    ## [{{ version | trim_start_matches(pat="v") }}] - {{ timestamp | date(format="%Y-%m-%d") }}
{% else %}\
    ## [Unreleased]
{% endif %}\
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {{ commit.message | upper_first }}\
    {% endfor %}
{% endfor %}
"""
trim = true

[git]
conventional_commits = true
filter_unconventional = true
split_commits = false
commit_preprocessors = [
    { pattern = '\((\w+\s)?#([0-9]+)\)', replace = "" },
]
```

#### cog - 版本管理工具

- 自动版本号推断
- 语义化版本
- Git标签管理

**替代方案**: 使用git-cliff内置的bump功能

### 1.4 开发工作流

#### 日常开发

```bash
# 功能开发
git commit -m "feat(serial): add timeout support"

# Bug修复
git commit -m "fix(protocol): handle empty response correctly"

# 文档更新
git commit -m "docs(readme): update installation instructions"
```

#### 发布流程

```bash
# 1. 合并所有PR到master
# 2. 触发version-bump workflow
gh workflow run version-bump.yml

# 3. 检查生成的版本号和changelog
git diff

# 4. 确认后创建PR
git push origin bump-v1.2.3

# 5. 合并PR后自动创建release
# 6. 自动构建和发布到所有渠道
```

### 1.5 文件结构

```
.github/
  workflows/
    commit-lint.yml        # 新增
    version-bump.yml       # 新增
    release.yml            # 修改（增强）
    ci.yml                 # 保持不变
  commitlint.config.js    # 新增
  cliff.toml              # 新增（changelog模板）

scripts/
  package/
    release.sh              # 发布辅助脚本
    verify-release.sh       # 发布验证脚本
    prepare-release.sh      # 发布准备脚本
    build.sh                # 构建脚本（统一接口）
  test/
    integration/
      test-release.sh       # 发布后集成测试

CHANGELOG.md              # 自动生成
```

### 1.6 验收标准

- [ ] 所有commit遵循Conventional Commits规范
- [ ] PR必须通过commitlint检查
- [ ] CHANGELOG.md自动生成且格式正确
- [ ] 版本号自动推断准确
- [ ] release workflow能成功创建GitHub Release
- [ ] 所有平台二进制构建成功

---

## 📦 阶段2：包管理器集成（2-3周）

### 目标

在阶段1基础上，扩展支持主流系统包管理器，为用户提供便捷的安装方式。

### 2.1 Homebrew集成

#### 策略

创建专用的tap仓库：`serial-cli-homebrew`

#### 文件结构

```
serial-cli-homebrew/          # 独立仓库
  Formula/
    serial-cli.rb             # CLI formula
  README.md
  LICENSE
```

#### Formula示例

```ruby
class SerialCli < Formula
  desc "Universal serial port CLI tool optimized for AI interaction"
  homepage "https://github.com/zazac-zhang/serial_cli"
  url "https://github.com/zazac-zhang/serial_cli/archive/refs/tags/v{version}.tar.gz"
  sha256 "{sha256_from_release}"
  license any_of: ["MIT", "Apache-2.0"]

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "."
  end

  test do
    system "#{bin}/serial-cli", "--version"
  end
end
```

#### 自动化流程

在`release.yml`中添加job：
```yaml
homebrew-release:
  name: Update Homebrew Tap
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Update formula
      env:
        GH_TOKEN: ${{ secrets.GH_TOKEN }}
      run: |
        VERSION=${{ github.ref_name }}
        SHA256_LINUX=$(sha256sum target/x86_64-unknown-linux-gnu/release/serial-cli | awk '{print $1}')

        git clone https://github.com/zazac-zhang/serial-cli-homebrew.git
        cd serial-cli-homebrew

        # 更新版本号
        sed -i "s/v{version}/$VERSION/g" Formula/serial-cli.rb
        sed -i "s/{sha256_from_release}/$SHA256_LINUX/g" Formula/serial-cli.rb

        git config user.name "github-actions[bot]"
        git config user.email "github-actions[bot]@users.noreply.github.com"
        git commit -am "Bump to $VERSION"
        git push
```

### 2.2 Scoop集成

#### 策略

创建自定义bucket：`serial-cli-scoop`

#### 文件结构

```
serial-cli-scoop/            # 独立仓库
  bucket/
    serial-cli.json          # CLI manifest
  README.md
```

#### Manifest示例

```json
{
  "version": "{version}",
  "description": "Universal serial port CLI tool",
  "homepage": "https://github.com/zazac-zhang/serial_cli",
  "license": "MIT OR Apache-2.0",
  "url": "https://github.com/zazac-zhang/serial_cli/releases/download/v{version}/serial-cli-windows-x86_64.exe",
  "hash": "{sha256_from_release}",
  "bin": "serial-cli-windows-x86_64.exe",
  "post_install": [
    "Rename-Item -Path \"$dir\\serial-cli-windows-x86_64.exe\" -NewName \"$dir\\serial-cli.exe\""
  ]
}
```

#### 自动化流程

```yaml
scoop-release:
  name: Update Scoop Bucket
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Update bucket
      env:
        GH_TOKEN: ${{ secrets.GH_TOKEN }}
      run: scripts/package/update_scoop.sh
```

### 2.3 AUR集成

#### 策略

使用AUR的Git仓库系统，创建辅助脚本生成文件

#### 文件结构

```
serial-cli-aur/              # 独立仓库（模拟AUR结构）
  PKGBUILD                   # 主构建文件
  .SRCINFO                   # 元数据
  README.md
```

#### PKGBUILD示例

```bash
pkgname=serial-cli
pkgver={version}
pkgrel=1
pkgdesc="Universal serial port CLI tool optimized for AI interaction"
arch=('x86_64' 'aarch64')
url="https://github.com/zazac-zhang/serial_cli"
license=('MIT' 'Apache')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/zazac-zhang/serial_cli/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('{sha256_from_release}')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/serial-cli" "$pkgdir/usr/bin/serial-cli"
}
```

#### 自动化流程

由于AUR限制，提供半自动化方案：

```yaml
aur-prepare:
  name: Prepare AUR Files
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Generate PKGBUILD
      run: scripts/package/generate_aur.sh
    - name: Upload artifacts
      uses: actions/upload-artifact@v4
      with:
        name: aur-files
        path: aur/
```

### 2.4 统一发布流程

#### 增强的release.yml工作流

```yaml
# 在现有release.yml后添加jobs

homebrew-release:
  name: Update Homebrew Tap
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Update formula
      run: scripts/package/update_homebrew.sh

scoop-release:
  name: Update Scoop Bucket
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - name: Update bucket
      run: scripts/package/update_scoop.sh

aur-prepare:
  name: Prepare AUR Files
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - name: Generate PKGBUILD
      run: scripts/package/generate_aur.sh
```

### 2.5 新增脚本

#### `scripts/package/` 目录扩展

```bash
update_homebrew.sh       # 更新Homebrew formula
update_scoop.sh          # 更新Scoop bucket
generate_aur.sh          # 生成AUR文件
verify_packages.sh       # 验证所有包管理器文件
test_install.sh          # 测试各平台安装
```

#### update_homebrew.sh 示例

```bash
#!/usr/bin/env bash
set -euo pipefail

VERSION=$1
SHA256=$2

TAP_REPO="https://github.com/zazac-zhang/serial-cli-homebrew.git"
TMP_DIR=$(mktemp -d)

git clone "$TAP_REPO" "$TMP_DIR"
cd "$TMP_DIR"

# 更新formula
sed -i "s/v{version}/$VERSION/g" Formula/serial-cli.rb
sed -i "s/{sha256_from_release}/$SHA256/g" Formula/serial-cli.rb

# 提交和推送
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"
git commit -am "Bump to $VERSION"
git push

rm -rf "$TMP_DIR"
```

### 2.6 发布检查清单

#### 在release.yml中添加验证步骤

```yaml
verify-release:
  name: Verify Release
  needs: [build, release]
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Run verification
      run: scripts/package/verify_packages.sh
```

#### 检查项

- [ ] 版本号格式正确（符合semver）
- [ ] CHANGELOG.md已更新且格式正确
- [ ] 所有平台二进制构建成功
- [ ] SHA256校验和已生成并验证
- [ ] Homebrew formula语法正确
- [ ] Scoop manifest格式正确
- [ ] AUR PKGBUILD可用
- [ ] 在至少2个平台测试安装成功

### 2.7 验收标准

- [ ] Homebrew安装成功：`brew install serial-cli`
- [ ] Scoop安装成功：`scoop bucket add serial-cli && scoop install serial-cli`
- [ ] AUR文件生成正确，可手动提交
- [ ] 所有包管理器的版本号同步
- [ ] 更新流程自动化，无需手动编辑
- [ ] 提供清晰的安装文档

---

## 📦 阶段3：GUI独立发布流程（1-2周）

### 目标

为GUI建立独立的发布轨道，支持更灵活的开发节奏，同时保持与CLI版本的关联。

### 3.1 GUI版本管理策略

#### 版本号规则

```
CLI版本: v1.2.3
GUI版本: v1.2.3-gui.1
```

#### 版本对应关系

- GUI跟随CLI的主版本号和次版本号
- GUI有自己的修订号（独立迭代）
- 示例：
  - CLI v1.2.0 → GUI v1.2.0-gui.0（初始版本）
  - CLI v1.2.0 → GUI v1.2.0-gui.1（GUI修复）
  - CLI v1.3.0 → GUI v1.3.0-gui.0（新版本）

#### 版本同步策略

```bash
# GUI开发初期：跟随CLI版本
CLI v1.2.0 + GUI功能完成 → GUI v1.2.0-gui.0

# GUI独立迭代：修复bug
GUI v1.2.0-gui.0 + bug修复 → GUI v1.2.0-gui.1

# 新CLI版本：GUI重新开始
CLI v1.3.0 + GUI功能更新 → GUI v1.3.0-gui.0
```

### 3.2 GUI独立工作流

#### 新增 `gui-release.yml` 工作流

```yaml
name: GUI Release

on:
  workflow_dispatch:
    inputs:
      cli_version:
        description: 'CLI base version (e.g., v1.2.3)'
        required: true
      gui_increment:
        description: 'GUI version increment'
        required: true
        default: 'patch'
        type: choice
        options:
          - major
          - minor
          - patch

jobs:
  build-gui:
    name: Build GUI for ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: serial-cli-gui-linux-amd64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: serial-cli-gui-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: serial-cli-gui-windows-amd64-setup.exe

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install GUI dependencies
        run: |
          cd src-ui
          npm install

      - name: Build GUI
        run: |
          cd src-tauri
          cargo tauri build --target ${{ matrix.target }}

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: |
            src-tauri/target/${{ matrix.target }}/release/bundle/*
```

### 3.3 GUI构建优化

#### Tauri配置优化

```json
// src-tauri/tauri.conf.json
{
  "bundle": {
    "identifier": "com.serialcli.gui",
    "icon": ["icons/icon.png"],
    "category": "Developer Tool",
    "shortDescription": "Serial Port Management Tool",
    "longDescription": "Cross-platform serial port communication tool with GUI",
    "targets": ["dmg", "nsis", "appimage", "deb"],
    "externalBin": [],
    "copyright": "",
    "license": "MIT OR Apache-2.0",
    "macOS": {
      "minimumSystemVersion": "10.13"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

#### 构建优化策略

**Linux**:
- AppImage（通用，x86_64 + aarch64）
- deb包（Debian系）
- RPM包（RedHat系）

**macOS**:
- DMG安装包（x86_64 + aarch64）
- 可选：Universal Binary

**Windows**:
- NSIS安装程序
- 便携版zip

### 3.4 GUI发布流程

#### 发布触发方式

```bash
# 1. GUI功能完成后，手动触发workflow
gh workflow run gui-release.yml \
  -f cli_version=v1.2.3 \
  -f gui_increment=patch

# 2. 审查构建产物
# 3. 确认后创建GitHub Release
# 4. 自动上传安装包
```

#### Release描述模板

```markdown
## Serial CLI GUI v{version}

**基于 CLI v{cli_version}**

### 📥 下载方式

**macOS (Apple Silicon)**
- [DMG安装包](serial-cli-gui-macos-arm64.dmg) - 推荐使用
- [校验和](serial-cli-gui-macos-arm64.dmg.sha256)

**macOS (Intel)**
- [DMG安装包](serial-cli-gui-macos-x86_64.dmg)
- [校验和](serial-cli-gui-macos-x86_64.dmg.sha256)

**Windows**
- [安装程序](serial-cli-gui-windows-amd64-setup.exe) - 推荐
- [便携版](serial-cli-gui-windows-amd64.zip)
- [校验和](serial-cli-gui-windows-amd64.exe.sha256)

**Linux (x86_64)**
- [AppImage](serial-cli-gui-linux-amd64.AppImage) - 通用版本
- [deb包](serial-cli-gui_1.2.3_amd64.deb) - Debian/Ubuntu
- [RPM包](serial-cli-gui-1.2.3-1.x86_64.rpm) - Fedora/RHEL

### ✨ 功能特性

- 🖥️ 图形化串口管理
- 📊 实时数据监控
- 📜 Lua脚本编辑器
- 🔧 协议配置界面
- 🎨 赛博工业风格界面

### 🔄 变更内容

**GUI特定变更**:
{gui_specific_changes}

**完整功能列表**:
查看 [CLI Release v{cli_version}](https://github.com/zazac-zhang/serial_cli/releases/tag/v{cli_version})

### 📚 安装指南

**macOS**:
```bash
# 下载DMG并安装
hdiutil attach serial-cli-gui-macos-arm64.dmg
cp -r /Volumes/Serial\ CLI\ GUI/Serial\ CLI\ GUI.app /Applications/
```

**Windows**:
```powershell
# 运行安装程序
.\serial-cli-gui-windows-amd64-setup.exe
```

**Linux**:
```bash
# AppImage（推荐）
chmod +x serial-cli-gui-linux-amd64.AppImage
./serial-cli-gui-linux-amd64.AppImage

# 或安装deb包
sudo dpkg -i serial-cli-gui_1.2.3_amd64.deb
```

### 🐛 问题反馈

[报告问题](https://github.com/zazac-zhang/serial_cli/issues)
```

### 3.5 GUI脚本和工具

#### `scripts/gui/` 目录结构

```
scripts/gui/
  build.sh              # GUI构建脚本
  test.sh               # GUI功能测试
  prepare-release.sh    # 发布准备
  update-version.sh     # 更新版本号
  icons/
    generate.sh         # 图标生成脚本
```

#### 版本管理脚本

```bash
#!/usr/bin/env bash
# scripts/gui/update-version.sh

set -euo pipefail

CLI_VERSION=$1  # v1.2.3
GUI_INCREMENT=$2  # patch/minor/major

# 解析CLI版本
MAJOR=$(echo $CLI_VERSION | cut -d. -f1 | sed 's/v//')
MINOR=$(echo $CLI_VERSION | cut -d. -f2)
PATCH=$(echo $CLI_VERSION | cut -d. -f3)

# 获取GUI独立版本号
GUI_PATCH=$(git tag --list "v${MAJOR}.${MINOR}.*-gui.*" | wc -l)
GUI_VERSION="v${MAJOR}.${MINOR}.${PATCH}-gui.${GUI_PATCH}"

# 更新Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${GUI_VERSION}\"/" src-tauri/Cargo.toml
rm -f src-tauri/Cargo.toml.bak

# 更新package.json
cd src-ui
npm version --no-git-tag-version "${GUI_VERSION}"
cd ..

echo "Updated GUI version to ${GUI_VERSION}"
```

#### 构建脚本

```bash
#!/usr/bin/env bash
# scripts/gui/build.sh

set -euo pipefail

TARGET=${1:-""}

echo "Building Serial CLI GUI..."

# 构建前端
echo "Building frontend..."
cd src-ui
npm install
npm run build
cd ..

# 构建Tauri应用
echo "Building Tauri application..."
cd src-tauri
if [ -n "$TARGET" ]; then
    cargo tauri build --target "$TARGET"
else
    cargo tauri build
fi

echo "✓ Build complete"
echo "Artifacts: src-tauri/target/release/bundle/"
```

### 3.6 GUI测试策略

#### 安装测试脚本

```bash
#!/usr/bin/env bash
# scripts/gui/test-install.sh

set -euo pipefail

test_dmg() {
    local dmg_file=$1
    echo "Testing macOS DMG: $dmg_file"

    # 挂载DMG
    local mount_point=$(hdiutil attach "$dmg_file" | grep -o '/Volumes/.*')

    # 检查应用是否存在
    if [ ! -d "$mount_point/Serial CLI GUI.app" ]; then
        echo "✗ Application not found in DMG"
        hdiutil detach "$mount_point"
        return 1
    fi

    # 卸载
    hdiutil detach "$mount_point"
    echo "✓ DMG test passed"
    return 0
}

test_nsis() {
    local exe_file=$1
    echo "Testing Windows NSIS: $exe_file"

    if [ ! -f "$exe_file" ]; then
        echo "✗ Setup file not found"
        return 1
    fi

    # 检查文件大小（应该 > 10MB）
    local size=$(stat -f%z "$exe_file" 2>/dev/null || stat -c%s "$exe_file")
    if [ "$size" -lt 10485760 ]; then
        echo "✗ Setup file too small: $size bytes"
        return 1
    fi

    echo "✓ NSIS test passed"
    return 0
}

test_appimage() {
    local appimage=$1
    echo "Testing Linux AppImage: $appimage"

    if [ ! -f "$appimage" ]; then
        echo "✗ AppImage not found"
        return 1
    fi

    # 测试可执行权限
    chmod +x "$appimage"

    # 测试版本信息
    if ! "$appimage" --version; then
        echo "✗ AppImage version check failed"
        return 1
    fi

    echo "✓ AppImage test passed"
    return 0
}

# 主测试流程
main() {
    local artifact_dir=$1

    echo "Starting GUI installation tests..."

    # 测试所有平台
    find "$artifact_dir" -name "*.dmg" -exec test_dmg {} \;
    find "$artifact_dir" -name "*setup.exe" -exec test_nsis {} \;
    find "$artifact_dir" -name "*.AppImage" -exec test_appimage {} \;

    echo "All tests completed"
}

main "$@"
```

### 3.7 GUI发布检查清单

#### GUI特有的检查项

- [ ] GUI版本号格式正确（vX.Y.Z-gui.N）
- [ ] CLI版本号正确关联
- [ ] 所有平台安装包构建成功
- [ ] 在目标平台测试安装
- [ ] 基本功能可用：
  - [ ] 端口列表显示
  - [ ] 打开/关闭端口
  - [ ] 数据监控
  - [ ] Lua脚本执行
- [ ] 安装包大小合理（<50MB）
- [ ] 图标和元数据正确
- [ ] 许可证包含在安装包中
- [ ] SHA256校验和生成

### 3.8 验收标准

- [ ] GUI可以独立发布，不依赖CLI发布
- [ ] 版本号管理清晰，与CLI版本关联明确
- [ ] 所有平台安装包可正常安装和运行
- [ ] 提供清晰的安装指南
- [ ] GUI发布流程文档完善
- [ ] 自动化测试覆盖主要功能

---

## 🏗️ 总体架构设计

### 系统架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    开发者工作流                              │
├─────────────────────────────────────────────────────────────┤
│  Conventional Commits → PR → commitlint → CI → 合并         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   阶段1: CI/CD基础                          │
├─────────────────────────────────────────────────────────────┤
│  • commitlint检查                                           │
│  • 自动版本推断（git-cliff/cog）                            │
│  • CHANGELOG生成                                            │
│  • 多平台构建（Linux/macOS/Windows）                         │
│  • SHA256校验和                                             │
│  • GitHub Release创建                                       │
│  • crates.io发布                                            │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   阶段2: 包管理器集成                        │
├─────────────────────────────────────────────────────────────┤
│  • Homebrew formula更新                                     │
│  • Scoop bucket更新                                         │
│  • AUR PKGBUILD生成                                         │
│  • 包管理器验证测试                                         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   阶段3: GUI独立发布                         │
├─────────────────────────────────────────────────────────────┤
│  • 手动触发GUI发布                                          │
│  • 独立版本管理                                             │
│  • Tauri打包（DMG/NSIS/AppImage/deb/RPM）                   │
│  • GUI功能测试                                              │
│  • 独立GitHub Release                                       │
└─────────────────────────────────────────────────────────────┘
```

### 数据流

```
Git Commits → Conventional Commits解析
    ↓
版本号推断 (semantic version)
    ↓
CHANGELOG生成 (git-cliff)
    ↓
触发Release Workflow
    ↓
并行构建:
├─ CLI二进制 (cargo build --release)
├─ GUI应用 (cargo tauri build)
└─ 包管理器文件 (Homebrew/Scoop/AUR)
    ↓
SHA256校验和计算
    ↓
创建GitHub Release
    ↓
并行发布:
├─ crates.io (CLI)
├─ Homebrew tap
├─ Scoop bucket
├─ AUR (手动)
└─ GUI Release
```

### 文件组织

```
serial-cli/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                  # 现有
│   │   ├── release.yml             # 增强（阶段1）
│   │   ├── commit-lint.yml         # 新增（阶段1）
│   │   ├── version-bump.yml        # 新增（阶段1）
│   │   └── gui-release.yml         # 新增（阶段3）
│   ├── commitlint.config.js        # 新增（阶段1）
│   └── cliff.toml                  # 新增（阶段1）
│
├── scripts/
│   ├── package/
│   │   ├── build.sh                # 新增（阶段1）
│   │   ├── prepare-release.sh      # 新增（阶段1）
│   │   ├── release.sh              # 新增（阶段1）
│   │   ├── verify-release.sh       # 新增（阶段1）
│   │   ├── update_homebrew.sh      # 新增（阶段2）
│   │   ├── update_scoop.sh         # 新增（阶段2）
│   │   ├── generate_aur.sh         # 新增（阶段2）
│   │   └── verify_packages.sh      # 新增（阶段2）
│   ├── gui/
│   │   ├── build.sh                # 新增（阶段3）
│   │   ├── prepare-release.sh      # 新增（阶段3）
│   │   ├── update-version.sh       # 新增（阶段3）
│   │   └── test-install.sh         # 新增（阶段3）
│   └── test/
│       └── integration/
│           └── test-release.sh     # 新增（阶段1）
│
├── src-tauri/                     # 现有（GUI后端）
├── src-ui/                        # 现有（GUI前端）
├── CHANGELOG.md                   # 自动生成（阶段1）
└── justfile                       # 现有（保持兼容）
```

---

## 📊 实施计划

### 时间线

| 阶段 | 时间 | 依赖 | 优先级 |
|------|------|------|--------|
| 阶段1: CI/CD基础 | 1-2周 | 无 | P0（必须） |
| 阶段2: 包管理器集成 | 2-3周 | 阶段1完成 | P1（重要） |
| 阶段3: GUI发布 | 1-2周 | 阶段1完成 | P2（次要） |

**注意**: 阶段2和阶段3可以并行开发，因为它们都只依赖阶段1。

### 里程碑

**Milestone 1: 规范化提交和版本管理**
- Conventional Commits规范实施
- 自动changelog生成
- 版本号自动推断

**Milestone 2: 多渠道发布**
- Homebrew集成
- Scoop集成
- AUR支持

**Milestone 3: GUI独立生态**
- GUI独立发布流程
- 平台原生安装包
- GUI版本管理

### 风险和缓解措施

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| Conventional Commits规范学习曲线 | 中 | 高 | 提供培训，逐步实施 |
| 包管理器API变更 | 低 | 中 | 使用稳定的API，定期检查 |
| GUI构建失败 | 中 | 低 | 充分测试，逐步支持平台 |
| 自动化发布误操作 | 高 | 低 | 多级检查，人工确认 |
| AUR维护复杂 | 中 | 中 | 提供脚本，半自动化 |

---

## ✅ 验收标准

### 总体验收标准

- [ ] 所有3个阶段功能正常工作
- [ ] CI/CD流程完全自动化
- [ ] 发布流程文档完善
- [ ] 用户可以轻松安装和更新
- [ ] 回滚机制存在且可工作

### 分阶段验收标准

#### 阶段1验收
- [ ] 100%的PR通过commitlint检查
- [ ] 版本号自动推断准确率>95%
- [ ] CHANGELOG自动生成且格式正确
- [ ] 发布成功率>98%

#### 阶段2验收
- [ ] Homebrew安装成功率>99%
- [ ] Scoop安装成功率>99%
- [ ] AUR文件生成正确
- [ ] 包管理器版本同步延迟<1小时

**阶段3验收**:
- [ ] GUI安装成功率>95%
- [ ] GUI基本功能可用（端口列表、打开/关闭端口、数据监控）
- [ ] 安装包大小<50MB
- [ ] 版本号管理清晰且与CLI版本关联正确
- [ ] 至少2个平台测试安装成功

---

## 📚 文档和培训

### 需要创建的文档

1. **CONTRIBUTING.md更新**
   - Conventional Commits规范
   - 开发工作流
   - 发布流程

2. **RELEASE.md（新建）**
   - 发布者指南
   - 故障排查
   - 回滚流程

3. **INTERNAL.md（新建）**
   - CI/CD架构说明
   - 脚本使用指南
   - 故障排查

4. **用户文档更新**
   - 安装指南（新增包管理器）
   - GUI安装指南
   - 更新指南

### 培训计划

**第1周**: 团队培训
- Conventional Commits工作坊
- CI/CD流程讲解
- 实战演练

**第2周**: 试运行
- 小规模测试发布
- 收集反馈
- 调整流程

---

## 🔄 后续优化方向

### 短期优化（1-3个月）

1. **监控和告警**
   - 发布失败告警
   - 构建时间监控
   - 成功率统计

2. **性能优化**
   - 并行构建优化
   - 缓存策略改进
   - 构建时间缩短

3. **用户体验**
   - 安装脚本简化
   - 错误信息改进
   - 文档完善

### 长期优化（3-6个月）

1. **扩展渠道**
   - Snap包
   - Chocolatey
   - Nix

2. **高级功能**
   - 自动回滚
   - 灰度发布
   - A/B测试

3. **安全增强**
   - 代码签名
   - SBOM生成
   - 安全扫描集成

---

## 📖 参考资源

### 工具和库

- [commitlint](https://github.com/conventional-changelog/commitlint)
- [git-cliff](https://github.com/orhun/git-cliff)
- [cog](https://github.com/cocogitto/cocogitto)
- [Tauri](https://tauri.app/)

### 最佳实践

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Scoop App Manifests](https://github.com/Scoopapps/Scoop/wiki/App-Manifests)

### 类似项目

- [ripgrep发布流程](https://github.com/BurntSushi/ripgrep)
- [fd发布流程](https://github.com/sharkdp/fd)
- [bat发布流程](https://github.com/sharkdp/bat)

---

## 📝 变更历史

| 版本 | 日期 | 变更内容 | 作者 |
|------|------|----------|------|
| 1.0.0 | 2026-04-06 | 初始设计 | Serial CLI Team |

---

**文档状态**: ✅ 设计完成，待评审
**下一步**: 创建实施计划

