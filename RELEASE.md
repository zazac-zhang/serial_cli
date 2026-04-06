# Release Guide

本指南介绍 Serial CLI 项目的发布流程。

## 前置条件

- 已安装 Rust 工具链
- 已安装 git-cliff: `cargo install git-cliff`
- 有 GitHub repository 的 write 权限

## 发布流程

### 1. 准备发布

```bash
# 准备新版本（例如 v1.2.3）
./scripts/package/prepare-release.sh v1.2.3

# 审查变更
git diff
git status

# 提交版本变更
git commit -am "chore: prepare release v1.2.3"
```

### 2. 运行验证

```bash
# 运行发布验证
./scripts/package/verify-release.sh

# 运行集成测试
./scripts/test/integration/test-release.sh
```

### 3. 创建 Release

```bash
# 创建并推送 tag
./scripts/package/release.sh v1.2.3
```

推送 tag 后，GitHub Actions 会自动：
1. 构建所有平台的二进制文件
2. 创建 GitHub Release
3. 发布到 crates.io

### 4. 验证发布

- [ ] GitHub Release 已创建
- [ ] 所有平台构建成功
- [ ] crates.io 发布成功
- [ ] CHANGELOG.md 已更新

## 回滚流程

如果发布失败或发现问题：

```bash
# 1. 删除 GitHub Release 和 tag
gh release delete v1.2.3 --cleanup-tag

# 2. 本地删除 tag（如果已创建）
git tag -d v1.2.3

# 3. 修复问题后重新发布
```

## 自动化版本管理

使用 `version-bump` workflow 自动管理版本：

```bash
# 手动触发（自动检测版本增量）
gh workflow run version-bump.yml

# 或指定增量类型
gh workflow run version-bump.yml -f increment=minor
```

Workflow 会：
1. 分析 commits 确定版本增量
2. 更新 Cargo.toml
3. 生成 CHANGELOG
4. 创建 Pull Request

审查并合并 PR 后，手动打 tag 发布。

## Conventional Commits 规范

提交信息格式：

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档变更
- `style`: 代码格式
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试
- `chore`: 构建/工具
- `ci`: CI/CD

### 示例

```bash
git commit -m "feat(cli): add protocol list command"
git commit -m "fix(protocol): handle empty response correctly"
git commit -m "docs(readme): update installation instructions"
```
